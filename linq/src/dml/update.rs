use crate::dml::CondExpr;

#[derive(Debug, Clone, PartialEq)]
pub struct Updater<'a> {
    pub table_name: &'a str,
    pub cols: Vec<&'a str>,
    pub cond: Option<CondExpr>,
}
