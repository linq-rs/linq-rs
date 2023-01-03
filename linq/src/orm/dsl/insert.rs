use std::marker::PhantomData;

use crate::{dml::Inserter, driver::InsertSupport, Variant};

use crate::orm::{find_col, table_primary_col, Column, ColumnValue, Table};

use super::Insert;

struct InsertCascade {
    table_name: &'static str,
    ref_col: Option<&'static str>,
    foreign_key_col: Option<&'static str>,
    foreign_key_col_value: Option<Variant>,
    cols: &'static [Column],
    values: Vec<ColumnValue>,
}

impl InsertCascade {
    pub async fn exec<'a, D>(self, d: &mut D) -> anyhow::Result<Vec<InsertCascade>>
    where
        D: InsertSupport<'a> + Sync + Send,
    {
        let (primary_col_name, auto_inc) =
            table_primary_col(self.cols).ok_or(anyhow::format_err!("Primary col not found"))?;

        let mut cols = vec![];
        let mut values = vec![];

        let mut insert_cascades = vec![];

        for value in self.values {
            match value {
                ColumnValue::Simple(col_name, value) => {
                    if let Variant::Null = value {
                        continue;
                    }

                    if col_name == primary_col_name && auto_inc {
                        continue;
                    }

                    cols.push(col_name);
                    values.push(value);
                }
                ColumnValue::OneToOne(col_name, values) => {
                    let col = find_col(self.cols, col_name)
                        .ok_or(anyhow::format_err!("OneToOne col {} not found", col_name))?;

                    match col {
                        Column::OneToOne(cascade) => insert_cascades.push(InsertCascade {
                            table_name: (cascade.table_name)(),
                            ref_col: Some(cascade.ref_col),
                            foreign_key_col: Some(cascade.foreign_key_col),
                            foreign_key_col_value: None,
                            cols: (cascade.table_cols)(),
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
                                insert_cascades.push(InsertCascade {
                                    table_name: (cascade.table_name)(),
                                    ref_col: Some(cascade.ref_col),
                                    foreign_key_col: Some(cascade.foreign_key_col),
                                    foreign_key_col_value: None,
                                    cols: (cascade.table_cols)(),
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

        if let Some(col_name) = self.foreign_key_col {
            cols.push(col_name);
            values.push(self.foreign_key_col_value.unwrap());
        }

        let mut insert_cascades_incompleted = vec![];
        let mut insert_cascades_completed = vec![];

        // First, try find cascade column ref_col bound value
        for mut cascade in insert_cascades {
            let ref_col = cascade.ref_col.unwrap();

            if let Some((idx, _)) = cols
                .iter()
                .enumerate()
                .find(|(_, col_name)| **col_name == ref_col)
            {
                cascade.foreign_key_col_value = Some(values[idx].clone());
                insert_cascades_completed.push(cascade);
            } else {
                insert_cascades_incompleted.push(cascade);
            }
        }

        let last_insert_id = d
            .insert(
                &Inserter {
                    table_name: self.table_name,
                    cols: cols.into(),
                },
                values,
            )
            .await?;

        // Then, try match ref_col to auto_inc primary col bound value.
        for mut cascade in insert_cascades_incompleted {
            let ref_col = cascade.ref_col.unwrap();

            if ref_col != primary_col_name {
                return Err(anyhow::format_err!(
                    "Cascade column {} ref col is null",
                    ref_col
                ));
            }

            cascade.foreign_key_col_value = Some(last_insert_id.into());

            insert_cascades_completed.push(cascade);
        }

        Ok(insert_cascades_completed)
    }
}

/// Cascade insert context structure .
pub struct InsertContext<T> {
    cascade: InsertCascade,
    _marked: PhantomData<T>,
}

impl<T> InsertContext<T>
where
    T: Table + Default,
{
    pub async fn exec<'a, D>(self, d: &mut D) -> anyhow::Result<()>
    where
        D: InsertSupport<'a> + Sync + Send,
    {
        let mut insert_stack = vec![self.cascade];

        // Cascade insert row
        while !insert_stack.is_empty() {
            let cascade = insert_stack.pop().unwrap();

            let mut next = cascade.exec(d).await?;

            insert_stack.append(&mut next);
        }

        Ok(())
    }
}

impl<T> Insert for T
where
    T: Table + Default,
{
    type Context = InsertContext<T>;
    fn insert(self) -> Self::Context {
        InsertContext {
            cascade: InsertCascade {
                table_name: T::table_name(),
                ref_col: None,
                foreign_key_col_value: None,
                foreign_key_col: None,
                cols: T::cols(),
                values: self.into_values(),
            },
            _marked: Default::default(),
        }
    }
}
