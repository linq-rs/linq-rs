use crate::Variant;

pub trait Table: Sized {
    /// Get table name
    fn table_name() -> &'static str;

    fn cols() -> &'static [Column];

    fn from_values(&mut self, values: Vec<ColumnValue>) -> anyhow::Result<()>;

    fn into_values(&mut self) -> Vec<ColumnValue>;
}

pub enum Column {
    Primary(&'static str, bool),
    Simple(&'static str),
    OneToOne(Cascade),
    OneToMany(Cascade),
}

impl From<&'static str> for Column {
    fn from(name: &'static str) -> Self {
        Column::Simple(name)
    }
}

pub struct Cascade {
    pub name: &'static str,
    pub ref_col: &'static str,
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
