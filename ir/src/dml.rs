use crate::{CondExpr, Limit, OrderBy, SelectColumns};

#[derive(Debug, Clone, PartialEq)]
pub struct Selecter<'a> {
    pub cols: SelectColumns<'a>,
    pub cond: Option<CondExpr>,
    pub limit: Option<Limit>,
    pub order_by: Option<OrderBy<'a>>,
}
