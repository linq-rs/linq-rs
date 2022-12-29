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
pub trait SelectSupport<'a> {
    type SelectResult: QueryIterator;
    /// Execute select stmt
    async fn select(&mut self, selecter: &dml::Selecter<'a>) -> anyhow::Result<Self::SelectResult>;
}

#[async_trait::async_trait]
pub trait UpdateSupport<'a> {
    /// Execute update stmt
    async fn update(
        &mut self,
        updater: &dml::Updater<'a>,
        values: Vec<Variant>,
    ) -> anyhow::Result<usize>;
}

#[async_trait::async_trait]
pub trait InsertSupport<'a> {
    /// Execute insert stmt
    async fn insert(
        &mut self,
        inserter: &dml::Inserter<'a>,
        values: Vec<Variant>,
    ) -> anyhow::Result<usize>;
}

#[async_trait::async_trait]
pub trait DeleteSupport<'a> {
    /// Execute delete stmt
    ///
    /// Returns deleted rows
    async fn delete(&mut self, deleter: &dml::Deleter<'a>) -> anyhow::Result<usize>;
}

#[async_trait::async_trait]
pub trait DDLSupport {
    /// Execute ddl stmts
    async fn exec_ddl<'a>(&mut self, ddls: &[ddl::DDL<'a>]) -> anyhow::Result<()>;
}
