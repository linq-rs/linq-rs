use chrono::{DateTime, NaiveTime, Utc};
use num::{BigInt, BigRational};

#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    Int(i64),
    BigInt(BigInt),
    Float(f64),
    BigFloat(BigRational),
    String(String),
    Bytes(Vec<u8>),
    DateTime(DateTime<Utc>),
    Timestamp(NaiveTime),
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

impl From<DateTime<Utc>> for Variant {
    fn from(v: DateTime<Utc>) -> Self {
        Variant::DateTime(v)
    }
}

impl From<NaiveTime> for Variant {
    fn from(v: NaiveTime) -> Self {
        Variant::Timestamp(v)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColumnType {
    Int,
    BigInt,
    Float,
    Decimal,
    String,
    Bytes,
    DateTime,
    Timestamp,
}
