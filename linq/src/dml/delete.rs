use crate::dml::CondExpr;

#[derive(Debug, Clone, PartialEq)]
pub struct Deleter<'a> {
    pub table_name: &'a str,
    pub cond: Option<CondExpr>,
}
