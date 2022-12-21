use crate::dml::CondExpr;

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
