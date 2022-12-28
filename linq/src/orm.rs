pub mod codegen;
mod table;
pub use table::*;

use crate::dml::{CondExpr, Limit};

pub trait SelectEx {
    type Context<'a>;
    fn select<'a>() -> Self::Context<'a>;
}

pub trait WhereEx {
    fn cond(self, cond: CondExpr) -> Self;
}

pub trait LimitEx {
    fn limit(self, count: usize) -> Self;
}

pub trait OffsetEx {
    fn offset(self, offset: usize) -> Self;
}

pub trait OrderByEx<'a> {
    fn order_by(self, col_name: &'a str, desc: bool) -> Self;
}

mod select;
pub use select::*;
