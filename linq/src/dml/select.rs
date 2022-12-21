use crate::dml::{CondExpr, From, Limit, OrderBy, SelectColumns};
use crate::variant::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Selecter<'a> {
    pub cols: SelectColumns<'a>,
    pub from: From<'a>,
    pub cond: Option<CondExpr>,
    pub limit: Option<Limit>,
    pub order_by: Option<OrderBy<'a>>,
}

#[async_trait::async_trait]
pub trait QueryResult {
    async fn next(&mut self) -> anyhow::Result<bool>;

    /// Get column value by offset id
    async fn get(&mut self, offset: usize) -> anyhow::Result<Variant>;

    /// Get column value by column name
    async fn get_by_name(&mut self, name: &str) -> anyhow::Result<Variant>;
}
