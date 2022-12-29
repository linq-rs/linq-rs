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

impl From<i64> for Variant {
    fn from(v: i64) -> Self {
        Variant::Int(v)
    }
}

impl From<i32> for Variant {
    fn from(v: i32) -> Self {
        Variant::Int(v as i64)
    }
}

impl From<usize> for Variant {
    fn from(v: usize) -> Self {
        Variant::Int(v as i64)
    }
}

impl From<BigInt> for Variant {
    fn from(v: BigInt) -> Self {
        Variant::BigInt(v)
    }
}

impl From<f64> for Variant {
    fn from(v: f64) -> Self {
        Variant::Float(v)
    }
}

impl From<f32> for Variant {
    fn from(v: f32) -> Self {
        Variant::Float(v as f64)
    }
}

impl From<BigRational> for Variant {
    fn from(v: BigRational) -> Self {
        Variant::BigFloat(v)
    }
}

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

impl TryFrom<Variant> for usize {
    type Error = anyhow::Error;
    fn try_from(value: Variant) -> Result<Self, Self::Error> {
        match value {
            Variant::Int(v) => Ok(v as usize),
            _ => Err(anyhow::format_err!("Variant type mismatch")),
        }
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

impl From<Timestamp> for Variant {
    fn from(v: Timestamp) -> Self {
        Variant::Timestamp(v)
    }
}

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
