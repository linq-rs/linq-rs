/// RQL compile time value
#[derive(Debug, Clone, PartialEq)]
pub enum IRValue {
    I64(i64),
    F64(f64),
    String(String),
    Bytes(Vec<u8>),
    Variant(String),
    Eval(String),
}
