use super::value::IRValue;

/// SQL dml language ir enum
pub enum DMLIR<'a> {
    Select(Selecter<'a>),
    Update(Updater<'a>),
    Insert(Insert<'a>),
    Delete(Deleter<'a>),
}

pub struct Deleter<'a> {
    pub table_name: &'a str,
    pub cond: Option<Condition<'a>>,
}

pub struct Insert<'a> {
    pub table_name: &'a str,
    pub cols: Vec<&'a str>,
    pub values: Vec<IRValue>,
}

pub struct Selecter<'a> {
    pub cols: Vec<SelectColumn<'a>>,
    pub cond: Option<Condition<'a>>,
    pub order: Option<Order<'a>>,
    pub limits: Option<Limits>,
}

pub struct Updater<'a> {
    pub table_name: &'a str,
    pub cols: Vec<&'a str>,
    pub values: Vec<IRValue>,
    pub cond: Option<Condition<'a>>,
}

pub struct From<'a> {
    pub table_name: &'a str,
    pub aliase: Option<&'a str>,
}

/// Query stmt col ir
pub struct SelectColumn<'a> {
    /// Table column name
    pub col_name: &'a str,
    /// More readable temporary column name
    pub aliase: Option<&'a str>,
}

pub enum Condition<'a> {
    Eq(&'a str, IRValue),
    Gt(&'a str, IRValue),
    Lt(&'a str, IRValue),
    Gte(&'a str, IRValue),
    Lte(&'a str, IRValue),
    Like(&'a str, IRValue),
    In(&'a str, Vec<IRValue>),
    And(Box<Condition<'a>>, Box<Condition<'a>>),
    Or(Box<Condition<'a>>, Box<Condition<'a>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Order<'a> {
    ASC(&'a str),
    DESC(&'a str),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Limits {
    // (offset,count)
    Offset((usize, usize)),
    Unlimits,
}
