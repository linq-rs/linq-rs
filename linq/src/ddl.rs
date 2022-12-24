use crate::{SqlType, Variant};

#[derive(Debug, Clone, PartialEq)]
pub enum DDL<'a> {
    Create(Create<'a>),
    Alter(Alter<'a>),
    Drop(&'a str),
    Truncate(&'a str),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Create<'a> {
    /// Create new table name
    pub table_name: &'a str,
    /// Create table column defines
    pub cols: Vec<Column<'a>>,
    /// Table constraints
    pub constraints: Vec<NamedConstraint<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Column<'a> {
    pub name: &'a str,
    pub col_type: SqlType,
    pub not_null: bool,
    pub default_value: Option<Variant>,
    pub primary: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamedConstraint<'a> {
    pub name: &'a str,
    pub constraint: Constraint<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constraint<'a> {
    Unique(Vec<&'a str>),
    Index(Vec<&'a str>),
    ForeignKey(Vec<&'a str>, &'a str, Vec<&'a str>),
    Check,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Alter<'a> {
    pub table_name: &'a str,
    pub expr: AlterExpr<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AlterExpr<'a> {
    AddColumn(Column<'a>),
    DropColumn(&'a str),
    AlterColumn(Column<'a>),
    AddConstraint(NamedConstraint<'a>),
    AlterConstraint(NamedConstraint<'a>),
    DropConstraint(&'a str),
    RenameTable(&'a str),
    RenameColumn(&'a str, &'a str),
    RenameConstraint(&'a str, &'a str),
}
