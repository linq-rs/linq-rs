#[derive(Debug, Clone, PartialEq)]
pub struct Columns<'a>(Vec<&'a str>);

impl<'a, 'b> From<&'b [&'a str]> for Columns<'a> {
    fn from(v: &'b [&'a str]) -> Self {
        Self(v.to_vec())
    }
}

impl<'a, 'b, const N: usize> From<&'b [&'a str; N]> for Columns<'a> {
    fn from(v: &'b [&'a str; N]) -> Self {
        Self(v.to_vec())
    }
}

impl<'a> From<Vec<&'a str>> for Columns<'a> {
    fn from(v: Vec<&'a str>) -> Self {
        Self(v)
    }
}

use crate::variant::Variant;

#[derive(Debug, Clone, PartialEq)]
pub enum CondOp {
    NotEq,
    Eq,
    Gt,
    Lt,
    Gte,
    Lte,
    Like,
    In,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CondParam {
    VariantList(Vec<Variant>),
    Variant(Variant),
    CondExpr(Box<CondExpr>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CondExpr {
    pub op: CondOp,
    pub lhs: CondParam,
    pub rhs: CondParam,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Deleter<'a> {
    pub table_name: &'a str,
    pub cond: CondExpr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Inserter<'a> {
    pub table_name: &'a str,
    pub cols: Columns<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Selecter<'a> {
    pub cols: SelectColumns<'a>,
    pub from: SelectFrom<'a>,
    pub cond: Option<CondExpr>,
    pub limit: Option<Limit>,
    pub order_by: Option<OrderBy<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectFrom<'a> {
    pub table_name: &'a str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrderBy<'a> {
    pub col_name: &'a str,
    pub desc: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Limit {
    pub count: usize,
    pub offset: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SelectColumns<'a> {
    // Match *
    All,

    NamedColumns(Vec<SelectNamedColumn<'a>>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectNamedColumn<'a> {
    pub name: &'a str,
    pub aliase: Option<&'a str>,
}

impl<'a> From<&'a str> for SelectNamedColumn<'a> {
    fn from(name: &'a str) -> Self {
        SelectNamedColumn { name, aliase: None }
    }
}

impl<'a> From<(&'a str, &'a str)> for SelectNamedColumn<'a> {
    fn from(pair: (&'a str, &'a str)) -> Self {
        SelectNamedColumn {
            name: pair.0,
            aliase: Some(pair.1),
        }
    }
}

impl<'a> From<Vec<&'a str>> for SelectColumns<'a> {
    fn from(cols: Vec<&'a str>) -> Self {
        SelectColumns::NamedColumns(cols.iter().map(|c| (*c).into()).collect())
    }
}

impl<'a> From<Vec<(&'a str, &'a str)>> for SelectColumns<'a> {
    fn from(cols: Vec<(&'a str, &'a str)>) -> Self {
        SelectColumns::NamedColumns(cols.iter().map(|c| (*c).into()).collect())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Updater<'a> {
    pub table_name: &'a str,
    pub cols: Columns<'a>,
    pub cond: CondExpr,
}
