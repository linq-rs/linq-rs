use std::marker::PhantomData;

use crate::{
    dml::{OrderBy, SelectColumns, SelectFrom, SelectNamedColumn, Selecter},
    QueryIterator, SelectSupport,
};

use super::{Limit, LimitEx, OffsetEx, OrderByEx, WhereEx};

use super::table::*;

pub trait SelectEx {
    type Context<'a>;
    fn select<'a>() -> Self::Context<'a>;
}

pub struct SelectOne<'a, T> {
    selecter: Selecter<'a>,
    _marked: PhantomData<T>,
}

impl<'a, T> SelectOne<'a, T>
where
    T: Table + Default,
{
    pub async fn exec<D>(&self, d: &mut D) -> anyhow::Result<T>
    where
        D: SelectSupport<'a> + Sync + Send,
    {
        let mut rows = d.select(&self.selecter).await?;

        let mut result: T = Default::default();

        while rows.next().await? {
            let mut values = vec![];

            for col in T::cols() {
                match col {
                    Column::Simple(name) => {
                        let value = rows.get_by_name(name).await?;
                        values.push(ColumnValue::Simple(&name, value));
                    }
                    Column::Primary(name, _) => {
                        let value = rows.get_by_name(name).await?;
                        values.push(ColumnValue::Simple(&name, value));
                    }
                    _ => continue,
                }
            }

            result.from_values(values)?;

            break;
        }

        Ok(result)
    }
}

pub struct SelectMany<'a, T> {
    selecter: Selecter<'a>,
    _marked: PhantomData<T>,
}

impl<'a, T> SelectMany<'a, T>
where
    T: Table + Default,
{
    pub async fn exec<D>(&self, d: &mut D) -> anyhow::Result<Vec<T>>
    where
        D: SelectSupport<'a> + Sync + Send,
    {
        let mut rows = d.select(&self.selecter).await?;

        let mut result: Vec<T> = Default::default();

        while rows.next().await? {
            let mut values = vec![];

            for col in T::cols() {
                match col {
                    Column::Simple(name) => {
                        let value = rows.get_by_name(name).await?;
                        values.push(ColumnValue::Simple(&name, value));
                    }
                    Column::Primary(name, _) => {
                        let value = rows.get_by_name(name).await?;
                        values.push(ColumnValue::Simple(&name, value));
                    }
                    _ => continue,
                }
            }

            let mut t: T = Default::default();

            t.from_values(values)?;

            result.push(t);
        }

        Ok(result)
    }
}

impl<T> SelectEx for T
where
    T: Table + Sync + Send,
{
    type Context<'a> = SelectOne<'a, T>;

    fn select<'a>() -> Self::Context<'a> {
        let mut cols = vec![];

        for col in T::cols() {
            match col {
                Column::Simple(name) => {
                    cols.push(SelectNamedColumn { name, aliase: None });
                }
                Column::Primary(name, _) => {
                    cols.push(SelectNamedColumn { name, aliase: None });
                }
                _ => continue,
            }
        }

        let context = SelectOne {
            selecter: Selecter {
                cols: SelectColumns::NamedColumns(cols),
                from: SelectFrom {
                    table_name: T::table_name(),
                },
                cond: None,
                limit: Some(Limit {
                    count: 1,
                    offset: None,
                }),
                order_by: None,
            },
            _marked: Default::default(),
        };

        context
    }
}

impl<'a, T> WhereEx for SelectOne<'a, T> {
    fn cond(mut self, cond: crate::dml::CondExpr) -> Self {
        self.selecter.cond = Some(cond);
        self
    }
}

impl<'a, T> OffsetEx for SelectOne<'a, T> {
    fn offset(mut self, offset: usize) -> Self {
        match self.selecter.limit {
            Some(limit) => {
                self.selecter.limit = Some(Limit {
                    count: limit.count,
                    offset: Some(offset),
                });
            }
            None => {
                self.selecter.limit = Some(Limit {
                    count: 0,
                    offset: Some(offset),
                });
            }
        }

        self
    }
}

impl<'a, T> OrderByEx<'a> for SelectOne<'a, T> {
    fn order_by(mut self, col_name: &'a str, desc: bool) -> Self {
        self.selecter.order_by = Some(OrderBy { col_name, desc });
        self
    }
}

impl<T> SelectEx for Vec<T>
where
    T: Table + Sync + Send,
{
    type Context<'a> = SelectMany<'a, T>;

    fn select<'a>() -> Self::Context<'a> {
        let mut cols = vec![];

        for col in T::cols() {
            match col {
                Column::Simple(name) => {
                    cols.push(SelectNamedColumn { name, aliase: None });
                }
                Column::Primary(name, _) => {
                    cols.push(SelectNamedColumn { name, aliase: None });
                }
                _ => continue,
            }
        }

        let context = SelectMany {
            selecter: Selecter {
                cols: SelectColumns::NamedColumns(cols),
                from: SelectFrom {
                    table_name: T::table_name(),
                },
                cond: None,
                limit: None,
                order_by: None,
            },
            _marked: Default::default(),
        };

        context
    }
}

impl<'a, T> LimitEx for SelectMany<'a, T> {
    fn limit(mut self, count: usize) -> Self {
        match self.selecter.limit {
            Some(limit) => {
                self.selecter.limit = Some(Limit {
                    count: count,
                    offset: limit.offset,
                });
            }
            None => {
                self.selecter.limit = Some(Limit {
                    count,
                    offset: None,
                });
            }
        }

        self
    }
}

impl<'a, T> WhereEx for SelectMany<'a, T> {
    fn cond(mut self, cond: crate::dml::CondExpr) -> Self {
        self.selecter.cond = Some(cond);
        self
    }
}

impl<'a, T> OffsetEx for SelectMany<'a, T> {
    fn offset(mut self, offset: usize) -> Self {
        match self.selecter.limit {
            Some(limit) => {
                self.selecter.limit = Some(Limit {
                    count: limit.count,
                    offset: Some(offset),
                });
            }
            None => {
                self.selecter.limit = Some(Limit {
                    count: 0,
                    offset: Some(offset),
                });
            }
        }

        self
    }
}

impl<'a, T> OrderByEx<'a> for SelectMany<'a, T> {
    fn order_by(mut self, col_name: &'a str, desc: bool) -> Self {
        self.selecter.order_by = Some(OrderBy { col_name, desc });
        self
    }
}
