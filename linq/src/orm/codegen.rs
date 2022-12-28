use crate::Variant;

use super::table::{ColumnValue, Table};

pub trait ColumnLike {
    fn into_column_value(&mut self, col_name: &'static str) -> ColumnValue;
    fn from_column_value(&mut self, value: ColumnValue) -> anyhow::Result<()>;
}

#[derive(Debug, Clone)]
pub struct Column<T>
where
    T: Into<Variant> + TryFrom<Variant>,
{
    pub value: Option<T>,
}

impl<T> Default for Column<T>
where
    T: Into<Variant> + TryFrom<Variant>,
{
    fn default() -> Self {
        Self { value: None }
    }
}

impl<T> ColumnLike for Column<T>
where
    T: Into<Variant> + TryFrom<Variant, Error = anyhow::Error>,
{
    fn into_column_value(&mut self, col_name: &'static str) -> ColumnValue {
        if let Some(v) = self.value.take() {
            ColumnValue::Variant(col_name, v.into())
        } else {
            ColumnValue::Variant(col_name, Variant::Null)
        }
    }

    fn from_column_value(&mut self, value: ColumnValue) -> anyhow::Result<()> {
        match value {
            ColumnValue::Variant(_, value) => {
                if let Variant::Null = value {
                    self.value = None;
                } else {
                    self.value = Some(value.try_into()?);
                }

                Ok(())
            }
            _ => Err(anyhow::format_err!("Column type mismatch")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OneToOne<T>
where
    T: Table,
{
    pub value: Option<T>,
}

impl<T> Default for OneToOne<T>
where
    T: Table,
{
    fn default() -> Self {
        Self { value: None }
    }
}

impl<T> ColumnLike for OneToOne<T>
where
    T: Table + Default,
{
    fn into_column_value(&mut self, col_name: &'static str) -> ColumnValue {
        if let Some(mut v) = self.value.take() {
            ColumnValue::Cascade(col_name, v.into_values())
        } else {
            ColumnValue::Variant(col_name, Variant::Null)
        }
    }

    fn from_column_value(&mut self, value: ColumnValue) -> anyhow::Result<()> {
        match value {
            ColumnValue::Cascade(_, values) => {
                let mut v: T = Default::default();

                v.from_values(values)?;

                self.value = Some(v);
                Ok(())
            }
            _ => Err(anyhow::format_err!("Column type mismatch")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OneToMany<T>
where
    T: Table,
{
    pub value: Option<Vec<T>>,
}

impl<T> Default for OneToMany<T>
where
    T: Table,
{
    fn default() -> Self {
        Self { value: None }
    }
}

impl<T> ColumnLike for OneToMany<T>
where
    T: Table + Default,
{
    fn into_column_value(&mut self, col_name: &'static str) -> ColumnValue {
        if let Some(v) = self.value.take() {
            let mut rows = vec![];

            for mut row in v {
                rows.push(row.into_values());
            }

            ColumnValue::CascadeMany(col_name, rows)
        } else {
            ColumnValue::Variant(col_name, Variant::Null)
        }
    }

    fn from_column_value(&mut self, value: ColumnValue) -> anyhow::Result<()> {
        match value {
            ColumnValue::CascadeMany(_, rows) => {
                let mut values = vec![];

                for row in rows {
                    let mut v: T = Default::default();

                    v.from_values(row)?;

                    values.push(v);
                }

                self.value = Some(values);

                Ok(())
            }
            _ => Err(anyhow::format_err!("Column type mismatch")),
        }
    }
}
