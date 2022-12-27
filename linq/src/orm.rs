use crate::{
    dml::{CondExpr, Limit},
    Variant,
};

pub trait Table: Sized {
    /// Get table name
    fn table_name() -> &'static str;

    fn cols() -> &'static [Column];

    fn write(&mut self, values: Vec<ColumnValue>) -> anyhow::Result<()>;

    fn read(&self) -> anyhow::Result<Vec<ColumnValue>>;
}

pub enum Column {
    WithName(&'static str),

    Cascade(Cascade),
}

impl From<&'static str> for Column {
    fn from(name: &'static str) -> Self {
        Column::WithName(name)
    }
}

pub struct Cascade {
    pub name: &'static str,
    pub reference_col: &'static str,
    pub table_name: &'static str,
    pub foreign_key_col: &'static str,
}

pub enum ColumnValue {
    Variant(&'static str, Variant),

    Cascade(&'static str, Vec<ColumnValue>),

    CascadeMany(&'static str, Vec<Vec<ColumnValue>>),
}

impl ColumnValue {
    pub fn col_name(&self) -> &'static str {
        match self {
            Self::Variant(name, _) => name,
            Self::Cascade(name, _) => name,
            Self::CascadeMany(name, _) => name,
        }
    }

    pub fn variant_value(&self) -> anyhow::Result<Variant> {
        match self {
            Self::Variant(_, value) => Ok(value.clone()),
            Self::Cascade(_, _) => Err(anyhow::format_err!("Column type mismatch")),
            Self::CascadeMany(_, _) => Err(anyhow::format_err!("Column type mismatch")),
        }
    }
}

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
