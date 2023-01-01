use std::marker::PhantomData;

use crate::{
    dml::{OrderBy, SelectColumns, SelectFrom, SelectNamedColumn, Selecter},
    driver::{QueryIterator, SelectSupport},
};

use crate::orm::{table::*, Limit, Offset, Order, Where};

use super::Select;

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

            return Ok(T::from_values(values)?);
        }

        unimplemented!()
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

            let t = T::from_values(values)?;

            result.push(t);
        }

        Ok(result)
    }
}

impl<T> Select for T
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
                limit: Some(crate::dml::Limit {
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

impl<'a, T> Where for SelectOne<'a, T> {
    type Context = SelectOne<'a, T>;
    fn cond(mut self, cond: crate::dml::CondExpr) -> Self::Context {
        self.selecter.cond = Some(cond);
        self
    }
}

impl<'a, T> Offset for SelectOne<'a, T> {
    type Context = SelectOne<'a, T>;
    fn offset(mut self, offset: usize) -> Self::Context {
        match self.selecter.limit {
            Some(limit) => {
                self.selecter.limit = Some(crate::dml::Limit {
                    count: limit.count,
                    offset: Some(offset),
                });
            }
            None => {
                self.selecter.limit = Some(crate::dml::Limit {
                    count: 0,
                    offset: Some(offset),
                });
            }
        }

        self
    }
}

impl<'a, T> Order<'a> for SelectOne<'a, T> {
    type Context = SelectOne<'a, T>;
    fn order_by(mut self, col_name: &'a str, desc: bool) -> Self::Context {
        self.selecter.order_by = Some(OrderBy { col_name, desc });
        self
    }
}

impl<T> Select for Vec<T>
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

impl<'a, T> Limit for SelectMany<'a, T> {
    type Context = SelectMany<'a, T>;
    fn limit(mut self, count: usize) -> Self {
        match self.selecter.limit {
            Some(limit) => {
                self.selecter.limit = Some(crate::dml::Limit {
                    count: count,
                    offset: limit.offset,
                });
            }
            None => {
                self.selecter.limit = Some(crate::dml::Limit {
                    count,
                    offset: None,
                });
            }
        }

        self
    }
}

impl<'a, T> Where for SelectMany<'a, T> {
    type Context = SelectMany<'a, T>;
    fn cond(mut self, cond: crate::dml::CondExpr) -> Self {
        self.selecter.cond = Some(cond);
        self
    }
}

impl<'a, T> Offset for SelectMany<'a, T> {
    type Context = SelectMany<'a, T>;
    fn offset(mut self, offset: usize) -> Self {
        match self.selecter.limit {
            Some(limit) => {
                self.selecter.limit = Some(crate::dml::Limit {
                    count: limit.count,
                    offset: Some(offset),
                });
            }
            None => {
                self.selecter.limit = Some(crate::dml::Limit {
                    count: 0,
                    offset: Some(offset),
                });
            }
        }

        self
    }
}

impl<'a, T> Order<'a> for SelectMany<'a, T> {
    type Context = SelectMany<'a, T>;
    fn order_by(mut self, col_name: &'a str, desc: bool) -> Self {
        self.selecter.order_by = Some(OrderBy { col_name, desc });
        self
    }
}
