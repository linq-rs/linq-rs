use crate::{DataType, Variant};

pub struct Create<'a> {
    /// Create new table name
    pub table_name: &'a str,
    /// Create table column defines
    pub cols: Vec<Column<'a>>,
    /// Table constraints
    pub constraints: Vec<NamedConstraint<'a>>,
}

pub struct Column<'a> {
    pub name: &'a str,
    pub col_type: DataType,
    pub col_type_len: Option<usize>,
    pub constraints: Vec<NamedConstraint<'a>>,
}

pub struct NamedConstraint<'a> {
    pub name: Option<&'a str>,
    pub constraint: Constraint<'a>,
}

pub enum Constraint<'a> {
    NotNull,
    Primary(&'a str, bool),
    Unique(Vec<&'a str>),
    Index(Vec<&'a str>),
    ForeignKey(Vec<&'a str>, &'a str, Vec<&'a str>),
    Check,
    Default(Variant),
}

pub struct Alter<'a> {
    pub table_name: &'a str,
    pub exprs: Vec<AlterExpr<'a>>,
}

pub enum AlterExpr<'a> {
    AddColumn(Column<'a>),
    DropColumn(&'a str),
    ModifyColumn((&'a str, Column<'a>)),
    AddConstraint(NamedConstraint<'a>),
    DropConstraint(NamedConstraint<'a>),
    Rename(&'a str),
    RenameColumn(&'a str, &'a str),
}
