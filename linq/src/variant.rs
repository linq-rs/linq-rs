#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    I64(i64),
    F64(f64),
    String(String),
    Bytes(Vec<u8>),
    Null,
}

impl From<i64> for Variant {
    fn from(v: i64) -> Self {
        Variant::I64(v)
    }
}

impl From<i32> for Variant {
    fn from(v: i32) -> Self {
        Variant::I64(v as i64)
    }
}

impl From<f64> for Variant {
    fn from(v: f64) -> Self {
        Variant::F64(v)
    }
}

impl From<f32> for Variant {
    fn from(v: f32) -> Self {
        Variant::F64(v as f64)
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

pub enum DataType {
    I64,
    F64,
    String,
    Bytes,
}
