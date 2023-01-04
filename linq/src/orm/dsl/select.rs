use std::marker::PhantomData;

use crate::{
    dml::{CondExpr, CondOp, CondParam, Limit, OrderBy, Selecter},
    driver::{QueryIterator, SelectSupport},
    orm::{find_col_value, Column, ColumnValue, Table},
    Select,
};

use super::Where;

#[allow(unused)]
struct CascadeSelecter<'a> {
    table_name: &'static str,
    cols: &'static [Column],
    cond: Option<CondExpr>,
    limits: Option<Limit>,
    order_by: Option<OrderBy<'a>>,
}

impl<'a> CascadeSelecter<'a> {
    #[async_recursion::async_recursion]
    pub async fn exec<D>(self, d: &mut D) -> anyhow::Result<Vec<Vec<ColumnValue>>>
    where
        D: SelectSupport<'a> + Sync + Send,
        'a: 'async_recursion,
    {
        let mut cols = vec![];

        for col in self.cols {
            match col {
                Column::Simple(col_name) => {
                    cols.push(*col_name);
                }
                Column::Primary(col_name, _) => {
                    cols.push(*col_name);
                }
                _ => {}
            }
        }

        let selecter = Selecter {
            from: self.table_name.into(),
            cols: cols.clone().into(),
            cond: self.cond,
            order_by: self.order_by,
            limit: self.limits,
        };

        let mut rows = d.select(&selecter).await?;

        let mut result = vec![];

        while rows.next().await? {
            let mut row_values = vec![];

            for col_name in &cols {
                let value = rows.get_by_name(col_name).await?;

                row_values.push(ColumnValue::Simple(*col_name, value));
            }

            for col in self.cols {
                match col {
                    Column::OneToOne(cascade) => {
                        let ref_col_value = find_col_value(&row_values, cascade.ref_col)
                            .ok_or(anyhow::format_err!(
                                "Cascade({}) ref_col {} is null",
                                cascade.name,
                                cascade.ref_col
                            ))?
                            .as_simple_value()?;

                        let cascade_selecter = CascadeSelecter {
                            table_name: (cascade.table_name)(),
                            cols: (cascade.table_cols)(),
                            cond: Some(CondExpr {
                                op: CondOp::Eq,
                                lhs: CondParam::Variant(cascade.foreign_key_col.into()),
                                rhs: CondParam::Variant(ref_col_value.clone()),
                            }),
                            limits: Some(Limit {
                                count: 1,
                                offset: None,
                            }),
                            order_by: None,
                        };

                        let mut result = cascade_selecter.exec(d).await?;

                        if !result.is_empty() {
                            row_values.push(ColumnValue::OneToOne(cascade.name, result.remove(0)));
                        }
                    }
                    Column::OneToMany(cascade) => {
                        let ref_col_value = find_col_value(&row_values, cascade.ref_col)
                            .ok_or(anyhow::format_err!(
                                "Cascade({}) ref_col {} is null",
                                cascade.name,
                                cascade.ref_col
                            ))?
                            .as_simple_value()?;

                        let cascade_selecter = CascadeSelecter {
                            table_name: (cascade.table_name)(),
                            cols: (cascade.table_cols)(),
                            cond: Some(CondExpr {
                                op: CondOp::Eq,
                                lhs: CondParam::Variant(cascade.foreign_key_col.into()),
                                rhs: CondParam::Variant(ref_col_value.clone()),
                            }),
                            limits: None,
                            order_by: None,
                        };

                        let result = cascade_selecter.exec(d).await?;

                        row_values.push(ColumnValue::OneToMany(cascade.name, result));
                    }
                    _ => {}
                }
            }

            result.push(row_values);
        }

        // Cascade select OneToOne/OntToMany cols

        Ok(result)
    }
}

//////////////////////////////////////////////////////////////////////////////////////
/// Select one

/// Select one row context.
pub struct SelectOne<'a, T> {
    selecter: CascadeSelecter<'a>,
    _marked: PhantomData<T>,
}

impl<'a, T> SelectOne<'a, T>
where
    T: Table,
{
    /// ending operator, execute query on target driver
    pub async fn exec<D>(self, d: &mut D) -> anyhow::Result<Option<T>>
    where
        D: SelectSupport<'a> + Sync + Send,
    {
        let mut values = self.selecter.exec(d).await?;

        if !values.is_empty() {
            Ok(Some(T::from_values(values.remove(0))?))
        } else {
            Ok(None)
        }
    }
}

impl<T> Select for T
where
    T: Table,
{
    type Context<'a> = SelectOne<'a, T>;
    fn select<'a>() -> Self::Context<'a> {
        SelectOne {
            selecter: CascadeSelecter {
                table_name: T::table_name(),
                cols: T::cols(),
                cond: None,
                limits: None,
                order_by: None,
            },
            _marked: Default::default(),
        }
    }
}

impl<'a, T> Where for SelectOne<'a, T> {
    type Context = SelectOne<'a, T>;

    fn cond(mut self, cond: CondExpr) -> Self::Context {
        self.selecter.cond = Some(cond);
        self
    }
}

impl<'a, T> super::Offset for SelectOne<'a, T> {
    type Context = SelectOne<'a, T>;

    fn offset(mut self, offset: usize) -> Self::Context {
        self.selecter.limits = Some(Limit {
            count: 1,
            offset: Some(offset),
        });

        self
    }
}

impl<'a, T> super::Order<'a> for SelectOne<'a, T> {
    type Context = SelectOne<'a, T>;

    fn order_by(mut self, col_name: &'a str, desc: bool) -> Self::Context {
        self.selecter.order_by = Some(OrderBy { col_name, desc });

        self
    }
}

//////////////////////////////////////////////////////////////////////////////////////
/// Select many
///

pub struct SelectMany<'a, T> {
    selecter: CascadeSelecter<'a>,
    _marked: PhantomData<T>,
}

impl<'a, T> SelectMany<'a, T>
where
    T: Table,
{
    /// Execute sql on target driver [`SelectSupport`]
    pub async fn exec<D>(self, d: &mut D) -> anyhow::Result<Vec<T>>
    where
        D: SelectSupport<'a> + Sync + Send,
    {
        let values = self.selecter.exec(d).await?;

        let mut result = vec![];

        for value in values {
            result.push(T::from_values(value)?);
        }

        Ok(result)
    }
}

impl<T> Select for Vec<T>
where
    T: Table,
{
    type Context<'a> = SelectMany<'a, T>;
    fn select<'a>() -> Self::Context<'a> {
        SelectMany {
            selecter: CascadeSelecter {
                table_name: T::table_name(),
                cols: T::cols(),
                cond: None,
                limits: None,
                order_by: None,
            },
            _marked: Default::default(),
        }
    }
}

impl<'a, T> Where for SelectMany<'a, T> {
    type Context = SelectMany<'a, T>;

    fn cond(mut self, cond: CondExpr) -> Self::Context {
        self.selecter.cond = Some(cond);
        self
    }
}

impl<'a, T> super::Limit for SelectMany<'a, T> {
    type Context = SelectMany<'a, T>;

    fn limit(mut self, count: usize, offset: usize) -> Self::Context {
        self.selecter.limits = Some(Limit {
            count: count,
            offset: Some(offset),
        });

        self
    }
}

impl<'a, T> super::Order<'a> for SelectMany<'a, T> {
    type Context = SelectMany<'a, T>;

    fn order_by(mut self, col_name: &'a str, desc: bool) -> Self::Context {
        self.selecter.order_by = Some(OrderBy { col_name, desc });

        self
    }
}
