use std::marker::PhantomData;

use crate::{
    dml::{CondExpr, CondOp, CondParam, Updater},
    driver::UpdateSupport,
    Variant,
};

use crate::orm::{ColumnValue, Table, TableEx, Where};

use super::Update;

/// Update context struct
pub struct UpdateContext<'a, T> {
    updater: Updater<'a>,
    values: Vec<Variant>,
    primary: Option<(&'a str, bool, Variant)>,
    _marked: PhantomData<T>,
}

impl<'a, T> UpdateContext<'a, T>
where
    T: Table + Default,
{
    pub async fn exec<D>(mut self, d: &mut D) -> anyhow::Result<usize>
    where
        D: UpdateSupport<'a> + Sync + Send,
    {
        if self.updater.cond.is_none() {
            if let Some((col_name, _, value)) = self.primary.take() {
                self.updater.cond = Some(CondExpr {
                    op: CondOp::Eq,
                    lhs: CondParam::Variant(Variant::String(col_name.to_owned())),
                    rhs: CondParam::Variant(value),
                });
            } else {
                return Err(anyhow::format_err!(
                    "Expect where clause or primary column value"
                ));
            }
        }

        d.update(&self.updater, self.values).await
    }
}

impl<T> Update for T
where
    T: Table + Default,
{
    type Context<'a> = UpdateContext<'a, T>;
    fn update<'a>(self) -> Self::Context<'a> {
        // let mut cond = None;

        let mut cols = vec![];
        let mut values = vec![];

        let (primary_col_name, auto_inc) = T::table_primary_col().expect("Not found primary col");

        let mut primary = None;

        for value in self.into_values() {
            match value {
                ColumnValue::Simple(col_name, variant) => {
                    if let Variant::Null = variant {
                        continue;
                    }

                    if primary_col_name == col_name {
                        primary = Some((col_name, auto_inc, variant.clone()));
                    }

                    cols.push(col_name);

                    values.push(variant);
                }
                _ => {
                    continue;
                }
            }
        }

        let context = UpdateContext {
            updater: Updater {
                table_name: T::table_name(),
                cols: cols.into(),
                cond: None,
            },
            values,
            primary,
            _marked: Default::default(),
        };

        context
    }
}

impl<'a, T> Where for UpdateContext<'a, T> {
    type Context = UpdateContext<'a, T>;
    fn cond(mut self, cond: crate::dml::CondExpr) -> Self {
        self.updater.cond = Some(cond);
        self
    }
}
