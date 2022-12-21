use crate::dml::CondExpr;

use super::Columns;

#[derive(Debug, Clone, PartialEq)]
pub struct Updater<'a> {
    pub table_name: &'a str,
    pub cols: Columns<'a>,
    pub cond: CondExpr,
}
