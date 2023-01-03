use crate::Variant;

pub trait Table: Sized {
    /// Get table name
    fn table_name() -> &'static str;

    fn cols() -> &'static [Column];

    fn from_values(values: Vec<ColumnValue>) -> anyhow::Result<Self>;

    fn into_values(self) -> Vec<ColumnValue>;
}

pub fn table_primary_col(cols: &'static [Column]) -> Option<(&'static str, bool)> {
    for col in cols {
        match col {
            Column::Primary(name, auto_inc) => return Some((name, *auto_inc)),
            _ => {}
        }
    }

    return None;
}
/// Find col by col_name from array
pub fn find_col<'a>(cols: &'a [Column], col_name: &'a str) -> Option<&'a Column> {
    cols.iter().find(|c| c.col_name() == col_name)
}

pub trait TableEx {
    fn table_primary_col() -> Option<(&'static str, bool)>;
}

impl<T> TableEx for T
where
    T: Table,
{
    fn table_primary_col() -> Option<(&'static str, bool)> {
        table_primary_col(Self::cols())
    }
}

pub enum Column {
    Primary(&'static str, bool),
    Simple(&'static str),
    OneToOne(Cascade),
    OneToMany(Cascade),
}

impl Column {
    pub fn col_name(&self) -> &'static str {
        match self {
            Column::Primary(name, _) => *name,
            Column::Simple(name) => *name,
            Self::OneToOne(cascade) => cascade.name,
            Self::OneToMany(cascade) => cascade.name,
        }
    }
}

impl From<&'static str> for Column {
    fn from(name: &'static str) -> Self {
        Column::Simple(name)
    }
}

pub struct Cascade {
    pub name: &'static str,
    pub ref_col: &'static str,
    pub table_name: fn() -> &'static str,
    pub foreign_key_col: &'static str,
    pub table_cols: fn() -> &'static [Column],
}

pub enum ColumnValue {
    Simple(&'static str, Variant),
    OneToOne(&'static str, Vec<ColumnValue>),
    OneToMany(&'static str, Vec<Vec<ColumnValue>>),
}

impl ColumnValue {
    pub fn col_name(&self) -> &'static str {
        match self {
            Self::Simple(name, _) => name,
            Self::OneToOne(name, _) => name,
            Self::OneToMany(name, _) => name,
        }
    }

    pub fn into_simple_value(&self) -> anyhow::Result<Variant> {
        match self {
            Self::Simple(_, value) => Ok(value.clone()),
            _ => Err(anyhow::format_err!("Column type mismatch")),
        }
    }
}
