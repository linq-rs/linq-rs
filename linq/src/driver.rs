use crate::ddl;
use crate::dml;
use crate::Variant;

#[async_trait::async_trait]
pub trait QueryIterator {
    async fn next(&mut self) -> anyhow::Result<bool>;

    /// Get column value by offset id
    async fn get(&mut self, offset: usize) -> anyhow::Result<Variant>;

    /// Get column value by column name
    async fn get_by_name(&mut self, name: &str) -> anyhow::Result<Variant>;
}

#[async_trait::async_trait]
pub trait Driver {
    type SelectResult: QueryIterator;

    /// Execute select stmt
    async fn select<'a>(selecter: &dml::Selecter<'a>) -> anyhow::Result<Self::SelectResult>;

    /// Execute update stmt
    async fn update<'a>(updater: &dml::Updater<'a>, values: &[Variant]) -> anyhow::Result<usize>;

    /// Execute insert stmt
    async fn insert<'a>(inserter: &dml::Inserter<'a>, values: &[Variant]) -> anyhow::Result<usize>;

    /// Execute delete stmt
    ///
    /// Returns deleted rows
    async fn delete<'a>(inserter: &dml::Deleter<'a>) -> anyhow::Result<usize>;

    /// Execute ddl stmts
    async fn exec_ddl<'a>(ddls: &[ddl::DDL<'a>]) -> anyhow::Result<()>;
}
