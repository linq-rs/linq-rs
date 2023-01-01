use crate::{
    orm::{ColumnValue, Table},
    DateTime, Timestamp, Variant,
};

/// Generic orm column trait.
///
/// Add serailze/deserialize ability via `LINQ` engine to
/// rust primitive types or generated table structures.
pub trait ColumnLike: Sized {
    /// Convert `Self` into [`ColumnValue`]
    fn into_column_value(self, col_name: &'static str) -> ColumnValue;

    /// Generate new type instance from [`ColumnValue`]
    fn from_column_value(value: ColumnValue) -> anyhow::Result<Self>;
}

/// Helper fn used by proc_macro, to reduce serialize/deserialize code generation difficulty.
///
/// Replace generated code
///
/// ```
/// # use linq_rs::orm::{ColumnLike,ColumnValue};
/// # let value = ColumnValue::Simple("test",linq_rs::Variant::Null);
/// Option::<i32>::from_column_value(value);
/// ```
/// with generated code
/// ```
/// # use linq_rs::orm::{ColumnLike,ColumnValue};
/// # let value = ColumnValue::Simple("test",linq_rs::Variant::Null);
/// linq_rs::orm::from_column_value::<Option<i32>>(value);
/// ```
/// The Column
/// [`TokenStream`](https://docs.rs/proc-macro2/latest/proc_macro2/struct.TokenStream.html)
/// `Option<i32>` can direct return by
/// [`syn::Field`](https://docs.rs/syn/latest/syn/struct.Field.html).
///
pub fn from_column_value<C>(value: ColumnValue) -> anyhow::Result<C>
where
    C: ColumnLike,
{
    C::from_column_value(value)
}

macro_rules! def_column_like {
    ($ty:ty) => {
        impl ColumnLike for $ty {
            fn into_column_value(self, col_name: &'static str) -> ColumnValue {
                ColumnValue::Simple(col_name, self.into())
            }

            fn from_column_value(value: ColumnValue) -> anyhow::Result<Self> {
                match value {
                    ColumnValue::Simple(col_name, value) => {
                        if let Variant::Null = value {
                            Err(anyhow::format_err!("Column({}) can't be none", col_name))
                        } else {
                            value.try_into()
                        }
                    }
                    _ => Err(anyhow::format_err!("Column type mismatch")),
                }
            }
        }
    };
}

def_column_like!(i8);
def_column_like!(i16);
def_column_like!(i32);
def_column_like!(i64);
def_column_like!(u8);
def_column_like!(u16);
def_column_like!(u32);
def_column_like!(u64);
def_column_like!(usize);
def_column_like!(String);
def_column_like!(Vec<u8>);
def_column_like!(DateTime);
def_column_like!(Timestamp);

impl<T> ColumnLike for Option<T>
where
    T: ColumnLike,
{
    fn into_column_value(self, col_name: &'static str) -> ColumnValue {
        match self {
            Some(v) => v.into_column_value(col_name),
            None => ColumnValue::Simple(col_name, Variant::Null),
        }
    }

    fn from_column_value(value: ColumnValue) -> anyhow::Result<Self> {
        match &value {
            ColumnValue::Simple(_, v) => {
                if let Variant::Null = v {
                    Ok(None)
                } else {
                    Ok(Some(T::from_column_value(value)?))
                }
            }
            _ => Err(anyhow::format_err!("Column type mismatch")),
        }
    }
}

impl<T> ColumnLike for T
where
    T: Table,
{
    fn into_column_value(self, col_name: &'static str) -> ColumnValue {
        ColumnValue::OneToOne(col_name, self.into_values())
    }

    fn from_column_value(value: ColumnValue) -> anyhow::Result<Self> {
        match value {
            ColumnValue::OneToOne(_, values) => Ok(T::from_values(values)?),
            _ => Err(anyhow::format_err!("Column type mismatch")),
        }
    }
}

impl<T> ColumnLike for Vec<T>
where
    T: Table,
{
    fn into_column_value(self, col_name: &'static str) -> ColumnValue {
        let mut rows = vec![];

        for row in self {
            rows.push(row.into_values());
        }

        ColumnValue::OneToMany(col_name, rows)
    }

    fn from_column_value(value: ColumnValue) -> anyhow::Result<Self> {
        match value {
            ColumnValue::OneToMany(_, rows) => {
                let mut values = vec![];

                for row in rows {
                    values.push(T::from_values(row)?);
                }

                Ok(values)
            }
            _ => Err(anyhow::format_err!("Column type mismatch")),
        }
    }
}
