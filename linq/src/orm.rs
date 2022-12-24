use crate::{dml::CondExpr, SelectSupport, Variant};

pub trait Table: Sized {
    /// Get table name
    fn table_name(&self) -> &'static str;

    fn cols(&self) -> &'static [&'static Column];

    fn write(&mut self, values: Vec<ColumnValue>) -> anyhow::Result<()>;

    fn read(&self) -> anyhow::Result<Vec<ColumnValue>>;
}

pub enum Column {
    WithName(&'static str),

    Cascade(Cascade),
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

#[async_trait::async_trait]
pub trait Select {
    async fn select<T, D>(&self, on: &mut D) -> anyhow::Result<T>
    where
        T: Table,
        D: SelectSupport + Sync + Send;
}

#[async_trait::async_trait]
impl Select for CondExpr {
    async fn select<T, D>(&self, on: &mut D) -> anyhow::Result<T>
    where
        T: Table,
        D: SelectSupport + Sync + Send,
    {
        unimplemented!()
    }
}
