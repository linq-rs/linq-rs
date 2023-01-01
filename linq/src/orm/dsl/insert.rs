use std::marker::PhantomData;

use crate::{dml::Inserter, driver::InsertSupport, Variant};

use crate::orm::{ColumnValue, Table, TableEx};

use super::Insert;

/// Update context struct
pub struct InsertContext<'a, T> {
    inserter: Inserter<'a>,
    values: Vec<Variant>,
    primary_check: bool,
    _marked: PhantomData<T>,
}

impl<'a, T> InsertContext<'a, T>
where
    T: Table + Default,
{
    pub async fn exec<D>(mut self, d: &mut D) -> anyhow::Result<usize>
    where
        D: InsertSupport<'a> + Sync + Send,
    {
        if !self.primary_check {
            return Err(anyhow::format_err!("Expect primary key value"));
        }

        d.insert(&self.inserter, self.values.drain(0..).collect())
            .await
    }
}

impl<T> Insert for T
where
    T: Table + Default,
{
    type Context<'a> = InsertContext<'a, T>;
    fn insert<'a>(self) -> Self::Context<'a> {
        // let mut cond = None;

        let mut cols = vec![];
        let mut values = vec![];

        let (primary_col_name, auto_inc) = T::table_primary_col().expect("Not found primary col");

        let mut primary_check = auto_inc;

        for value in self.into_values() {
            match value {
                ColumnValue::Simple(col_name, variant) => {
                    if let Variant::Null = variant {
                        continue;
                    }

                    if primary_col_name == col_name {
                        primary_check = true;

                        if auto_inc {
                            continue;
                        }
                    }

                    cols.push(col_name);

                    values.push(variant);
                }
                // ColumnValue::Primary(col_name, auto_inc, variant) => {
                //     if let Variant::Null = variant {
                //         continue;
                //     }

                //     primary_check = true;

                //     if auto_inc {
                //         continue;
                //     }

                //     cols.push(col_name);

                //     values.push(variant);
                // }
                _ => {
                    continue;
                }
            }
        }

        let context = InsertContext {
            inserter: Inserter {
                table_name: T::table_name(),
                cols: cols.into(),
            },
            values,
            primary_check,
            _marked: Default::default(),
        };

        context
    }
}
