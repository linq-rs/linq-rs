use std::fmt::Display;

use num::{BigInt, BigRational};

/// LINQ ir datetime type import from chrono.
pub type DateTime = chrono::DateTime<chrono::Utc>;

/// LINQ ir timestamp type import from chrono.
pub type Timestamp = chrono::NaiveTime;

/// LINQ ir basic type value enum
#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    Int(i64),
    BigInt(BigInt),
    Float(f64),
    BigFloat(BigRational),
    String(String),
    Bytes(Vec<u8>),
    DateTime(DateTime),
    Timestamp(Timestamp),
    Null,
}

impl Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(v) => {
                write!(f, "{}", v)
            }
            Self::BigInt(v) => {
                write!(f, "{}", v)
            }
            Self::Float(v) => {
                write!(f, "{}", v)
            }
            Self::BigFloat(v) => {
                write!(f, "{}", v)
            }
            Self::String(v) => {
                write!(f, "{}", v)
            }
            Self::Bytes(v) => {
                write!(f, "{:X?}", v)
            }
            Self::DateTime(v) => {
                write!(f, "{}", v)
            }
            Self::Timestamp(v) => {
                write!(f, "{}", v)
            }
            Self::Null => {
                write!(f, "NULL",)
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////
/// Integer like convert

/// Implement int convert macro
macro_rules! impl_int_convert {
    ($num: ty) => {
        impl From<$num> for Variant {
            fn from(v: $num) -> Self {
                Variant::Int(v as i64)
            }
        }

        impl TryFrom<Variant> for $num {
            type Error = anyhow::Error;
            fn try_from(value: Variant) -> Result<Self, Self::Error> {
                match value {
                    Variant::Int(v) => Ok(v as $num),
                    _ => Err(anyhow::format_err!("Variant type mismatch")),
                }
            }
        }
    };
}

impl_int_convert!(i8);
impl_int_convert!(i16);
impl_int_convert!(i32);
impl_int_convert!(i64);
impl_int_convert!(usize);
impl_int_convert!(u8);
impl_int_convert!(u16);
impl_int_convert!(u32);
impl_int_convert!(u64);

////////////////////////////////////////////////////////////////////////////////////
/// Float like convert

/// Implement float convert macro
macro_rules! impl_float_convert {
    ($num: ty) => {
        impl From<$num> for Variant {
            fn from(v: $num) -> Self {
                Variant::Float(v as f64)
            }
        }

        impl TryFrom<Variant> for $num {
            type Error = anyhow::Error;
            fn try_from(value: Variant) -> Result<Self, Self::Error> {
                match value {
                    Variant::Float(v) => Ok(v as $num),
                    _ => Err(anyhow::format_err!("Variant type mismatch")),
                }
            }
        }
    };
}

impl_float_convert!(f32);
impl_float_convert!(f64);

impl From<BigInt> for Variant {
    fn from(v: BigInt) -> Self {
        Variant::BigInt(v)
    }
}

impl From<BigRational> for Variant {
    fn from(v: BigRational) -> Self {
        Variant::BigFloat(v)
    }
}

////////////////////////////////////////////////////////////////////////////////////
/// StringLike convert

impl<'a> From<&'a str> for Variant {
    fn from(v: &'a str) -> Self {
        Variant::String(v.to_owned())
    }
}

impl From<String> for Variant {
    fn from(v: String) -> Self {
        Variant::String(v)
    }
}

impl TryFrom<Variant> for String {
    type Error = anyhow::Error;
    fn try_from(value: Variant) -> Result<Self, Self::Error> {
        match value {
            Variant::String(v) => Ok(v),
            _ => Err(anyhow::format_err!("Variant type mismatch")),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////
/// Bytes like convert

impl<'a> From<&'a [u8]> for Variant {
    fn from(v: &'a [u8]) -> Self {
        Variant::Bytes(v.to_owned())
    }
}

impl From<Vec<u8>> for Variant {
    fn from(v: Vec<u8>) -> Self {
        Variant::Bytes(v)
    }
}

////////////////////////////////////////////////////////////////////////////////////
/// Date like convert

impl From<DateTime> for Variant {
    fn from(v: DateTime) -> Self {
        Variant::DateTime(v)
    }
}

impl TryFrom<Variant> for DateTime {
    type Error = anyhow::Error;
    fn try_from(value: Variant) -> Result<Self, Self::Error> {
        match value {
            Variant::DateTime(datetime) => Ok(datetime),
            _ => Err(anyhow::format_err!("Variant type mismatch")),
        }
    }
}

impl From<Timestamp> for Variant {
    fn from(v: Timestamp) -> Self {
        Variant::Timestamp(v)
    }
}

/// end convert impl
////////////////////////////////////////////////////////////////////////////////////

/// LINQ ir basic type enum
#[derive(Debug, Clone, PartialEq)]
pub enum IrType {
    Int,
    BigInt,
    Float,
    Decimal,
    String,
    Bytes,
    DateTime,
    Timestamp,
}
