use std::marker::PhantomData;

use crate::{
    dml::{CondExpr, CondOp, CondParam, Updater},
    driver::UpdateSupport,
    orm::{find_col, table_primary_col, Column},
    Variant,
};

use crate::orm::{ColumnValue, Table, Where};

use super::Update;

struct UpdateCascade {
    table_name: &'static str,
    ref_col: Option<&'static str>,
    foreign_key_col: Option<&'static str>,
    foreign_key_col_value: Option<Variant>,
    cols: &'static [Column],
    values: Vec<ColumnValue>,
    cond: Option<CondExpr>,
}

impl UpdateCascade {
    pub async fn exec<'a, D>(self, d: &mut D) -> anyhow::Result<Vec<UpdateCascade>>
    where
        D: UpdateSupport<'a> + Sync + Send,
    {
        let (primary_col_name, auto_inc) =
            table_primary_col(self.cols).ok_or(anyhow::format_err!("Primary col not found"))?;

        let mut primary_col_value = None;

        let mut cols = vec![];
        let mut values = vec![];

        let mut update_cascades = vec![];

        for value in self.values {
            match value {
                ColumnValue::Simple(col_name, value) => {
                    if let Variant::Null = value {
                        continue;
                    }

                    if col_name == primary_col_name {
                        primary_col_value = Some(value.clone());

                        if auto_inc {
                            continue;
                        }
                    }

                    cols.push(col_name);
                    values.push(value);
                }
                ColumnValue::OneToOne(col_name, values) => {
                    let col = find_col(self.cols, col_name)
                        .ok_or(anyhow::format_err!("OneToOne col {} not found", col_name))?;

                    match col {
                        Column::OneToOne(cascade) => update_cascades.push(UpdateCascade {
                            table_name: (cascade.table_name)(),
                            ref_col: Some(cascade.ref_col),
                            foreign_key_col: Some(cascade.foreign_key_col),
                            foreign_key_col_value: None,
                            cols: (cascade.table_cols)(),
                            cond: None,
                            values,
                        }),
                        _ => {
                            return Err(anyhow::format_err!(
                                "Target col {} is not OneToOne column",
                                col_name
                            ));
                        }
                    }
                }
                ColumnValue::OneToMany(col_name, rows) => {
                    let col = find_col(self.cols, col_name)
                        .ok_or(anyhow::format_err!("OneToOne col {} not found", col_name))?;

                    match col {
                        Column::OneToMany(cascade) => {
                            for values in rows {
                                update_cascades.push(UpdateCascade {
                                    table_name: (cascade.table_name)(),
                                    ref_col: Some(cascade.ref_col),
                                    foreign_key_col: Some(cascade.foreign_key_col),
                                    foreign_key_col_value: None,
                                    cols: (cascade.table_cols)(),
                                    cond: None,
                                    values,
                                })
                            }
                        }
                        _ => {
                            return Err(anyhow::format_err!(
                                "Target col {} is not OneToMany column",
                                col_name
                            ));
                        }
                    }
                }
            }
        }

        for mut cascade in &mut update_cascades {
            let ref_col = cascade.ref_col.unwrap();

            if let Some((idx, _)) = cols
                .iter()
                .enumerate()
                .find(|(_, col_name)| **col_name == ref_col)
            {
                cascade.foreign_key_col_value = Some(values[idx].clone());
            } else {
                return Err(anyhow::format_err!(
                    "Cascade update({}) ref_col({}) is null",
                    cascade.table_name,
                    ref_col
                ));
            }
        }

        let cond = if self.cond.is_some() {
            self.cond
        } else {
            if let Some(col_name) = self.foreign_key_col {
                Some(CondExpr {
                    op: CondOp::Eq,
                    lhs: CondParam::Variant(col_name.into()),
                    rhs: CondParam::Variant(self.foreign_key_col_value.unwrap()),
                })
            } else if let Some(value) = primary_col_value {
                Some(CondExpr {
                    op: CondOp::Eq,
                    lhs: CondParam::Variant(primary_col_name.into()),
                    rhs: CondParam::Variant(value),
                })
            } else {
                return Err(anyhow::format_err!(
                    "Cascade table {} update miss cond clause",
                    self.table_name
                ));
            }
        };

        d.update(
            &Updater {
                table_name: self.table_name,
                cols: cols.into(),
                cond,
            },
            values,
        )
        .await?;

        // Then, try match ref_col to auto_inc primary col bound value.

        Ok(update_cascades)
    }
}

/// Update context struct
pub struct UpdateContext<T> {
    cascade: UpdateCascade,
    _marked: PhantomData<T>,
}

impl<T> UpdateContext<T>
where
    T: Table + Default,
{
    pub async fn exec<'a, D>(self, d: &mut D) -> anyhow::Result<()>
    where
        D: UpdateSupport<'a> + Sync + Send,
    {
        let mut update_stack = vec![self.cascade];

        // Cascade insert row
        while !update_stack.is_empty() {
            let cascade = update_stack.pop().unwrap();

            let mut next = cascade.exec(d).await?;

            update_stack.append(&mut next);
        }

        Ok(())
    }
}

impl<T> Update for T
where
    T: Table + Default,
{
    type Context = UpdateContext<T>;
    fn update(self) -> Self::Context {
        UpdateContext {
            cascade: UpdateCascade {
                table_name: T::table_name(),
                ref_col: None,
                foreign_key_col_value: None,
                foreign_key_col: None,
                cols: T::cols(),
                values: self.into_values(),
                cond: None,
            },
            _marked: Default::default(),
        }
    }
}

impl<T> Where for UpdateContext<T> {
    type Context = UpdateContext<T>;
    fn cond(mut self, cond: crate::dml::CondExpr) -> Self {
        self.cascade.cond = Some(cond);
        self
    }
}

pub struct UpdateAll<T> {
    cascades: Vec<UpdateCascade>,
    _marked: PhantomData<T>,
}

impl<T> UpdateAll<T>
where
    T: Table + Default,
{
    pub async fn exec<'a, D>(self, d: &mut D) -> anyhow::Result<()>
    where
        D: UpdateSupport<'a> + Sync + Send,
    {
        let mut update_stack = self.cascades;

        // Cascade insert row
        while !update_stack.is_empty() {
            let cascade = update_stack.pop().unwrap();

            let mut next = cascade.exec(d).await?;

            update_stack.append(&mut next);
        }

        Ok(())
    }
}

impl<T> Update for Vec<T>
where
    T: Table + Default,
{
    type Context = UpdateAll<T>;
    fn update(self) -> Self::Context {
        let mut cascades = vec![];

        for t in self {
            cascades.push(UpdateCascade {
                table_name: T::table_name(),
                ref_col: None,
                foreign_key_col_value: None,
                foreign_key_col: None,
                cols: T::cols(),
                values: t.into_values(),
                cond: None,
            });
        }

        UpdateAll {
            cascades,
            _marked: Default::default(),
        }
    }
}
