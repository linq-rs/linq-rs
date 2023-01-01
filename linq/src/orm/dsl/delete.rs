use std::marker::PhantomData;

use crate::{
    dml::{CondExpr, CondOp, CondParam, Deleter},
    driver::DeleteSupport,
    Variant,
};

use crate::orm::{ColumnValue, Table, Where};

use super::{DeleteObject, DeleteWhereCond};

/// Update context struct
pub struct DeleteObjectContext<'a, T> {
    table_name: &'a str,
    condexpr: Option<CondExpr>,
    _marked: PhantomData<T>,
}

impl<'a, T> DeleteObjectContext<'a, T>
where
    T: Table + Default,
{
    pub async fn exec<D>(mut self, d: &mut D) -> anyhow::Result<usize>
    where
        D: DeleteSupport<'a> + Sync + Send,
    {
        if let Some(condexpr) = self.condexpr.take() {
            d.delete(&Deleter {
                table_name: self.table_name,
                cond: condexpr,
            })
            .await
        } else {
            return Err(anyhow::format_err!("Expect primary column value"));
        }
    }
}

impl<T> DeleteObject for T
where
    T: Table + Default,
{
    type Context<'a> = DeleteObjectContext<'a, T>;
    fn delete<'a>(self) -> Self::Context<'a> {
        let mut condexpr = None;

        for value in self.into_values() {
            match value {
                ColumnValue::Simple(col_name, variant) => {
                    if let Variant::Null = variant {
                        continue;
                    }

                    condexpr = Some(CondExpr {
                        op: CondOp::Eq,
                        lhs: CondParam::Variant(Variant::String(col_name.to_owned())),
                        rhs: CondParam::Variant(variant),
                    });
                }
                _ => {
                    continue;
                }
            }
        }

        let context = DeleteObjectContext {
            table_name: T::table_name(),
            condexpr,
            _marked: Default::default(),
        };

        context
    }
}

impl<T> DeleteWhereCond for T
where
    T: Table + Default,
{
    type Context<'a> = DeleteWhereCondContext<'a>;
    fn delete<'a>() -> Self::Context<'a> {
        DeleteWhereCondContext {
            table_name: T::table_name(),
        }
    }
}

pub struct DeleteWhereCondContext<'a> {
    table_name: &'a str,
}

impl<'a> Where for DeleteWhereCondContext<'a> {
    type Context = DeleteCondContext<'a>;
    fn cond(self, cond: CondExpr) -> Self::Context {
        DeleteCondContext {
            deleter: Deleter {
                table_name: self.table_name,
                cond,
            },
        }
    }
}

pub struct DeleteCondContext<'a> {
    deleter: Deleter<'a>,
}

impl<'a> DeleteCondContext<'a> {
    pub async fn exec<D>(self, d: &mut D) -> anyhow::Result<usize>
    where
        D: DeleteSupport<'a> + Sync + Send,
    {
        d.delete(&self.deleter).await
    }
}
