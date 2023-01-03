//! Provide DSL methods to [`super::table::Table`] structures

use crate::dml::CondExpr;

/// Extension trait to support where condition.
/// # Examples
///
///
pub trait Where {
    type Context;
    fn cond(self, cond: CondExpr) -> Self::Context;
}

pub trait Limit {
    type Context;
    fn limit(self, count: usize) -> Self::Context;
}

pub trait Offset {
    type Context;
    fn offset(self, offset: usize) -> Self::Context;
}

pub trait Order<'a> {
    type Context;
    fn order_by(self, col_name: &'a str, desc: bool) -> Self::Context;
}

pub trait Select {
    type Context<'a>;
    fn select<'a>() -> Self::Context<'a>;
}

/// Extend [`Table`](super::Table) structures to support
/// DML [`INSERT`](https://www.w3schools.com/sql/sql_insert.asp) operation.
pub trait Insert {
    type Context;
    fn insert(self) -> Self::Context;
}

pub trait Update {
    type Context<'a>;
    fn update<'a>(self) -> Self::Context<'a>;
}

pub trait DeleteObject {
    type Context<'a>;
    fn delete<'a>(self) -> Self::Context<'a>;
}

pub trait DeleteWhereCond {
    type Context<'a>;
    fn delete<'a>() -> Self::Context<'a>;
}

mod select;
pub use select::*;

mod update;
pub use update::*;

mod insert;
pub use insert::*;

mod delete;
pub use delete::*;
