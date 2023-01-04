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

/// Trait to support executing [`SELECT`](https://www.w3schools.com/sql/sql_select.asp) expr.
///
/// # Examples
///
/// ```
/// use linq_rs::*;
///
/// async fn execute<D>(d: &mut D) -> anyhow::Result<D::SelectResult> where D: driver::SelectSupport<'static> {
///     let qir = rql! {
///        SELECT name,created_time FROM table WHERE id = 1 ORDER BY name DESC LIMIT 10 OFFSET 2;
///     };
///
///     d.select(&qir).await
/// }
///
/// ```
#[async_trait::async_trait]
pub trait SelectSupport<'a> {
    type SelectResult: QueryIterator + Send + Sync;

    /// Execute select stmt
    async fn select(&mut self, selecter: &dml::Selecter<'a>) -> anyhow::Result<Self::SelectResult>;
}

/// Trait to support executing [`UPDATE`](https://www.w3schools.com/sql/sql_update.asp) expr.
#[async_trait::async_trait]
pub trait UpdateSupport<'a> {
    /// Execute update stmt
    async fn update(
        &mut self,
        updater: &dml::Updater<'a>,
        values: Vec<Variant>,
    ) -> anyhow::Result<usize>;
}

/// Trait to support executing [`INSERT`](https://www.w3schools.com/sql/sql_insert.asp) expr.
#[async_trait::async_trait]
pub trait InsertSupport<'a> {
    /// Execute insert stmt
    async fn insert(
        &mut self,
        inserter: &dml::Inserter<'a>,
        values: Vec<Variant>,
    ) -> anyhow::Result<usize>;
}

/// Trait to support executing [`DELETE`](https://www.w3schools.com/sql/sql_delete.asp) expr.
#[async_trait::async_trait]
pub trait DeleteSupport<'a> {
    /// Execute delete stmt
    ///
    /// Returns deleted rows
    async fn delete(&mut self, deleter: &dml::Deleter<'a>) -> anyhow::Result<usize>;
}

/// Trait to support executing [`DDL`](https://www.javatpoint.com/dbms-sql-command) exprs.
#[async_trait::async_trait]
pub trait DDLSupport {
    /// Execute ddl stmts
    async fn exec_ddl<'a>(&mut self, ddls: &[ddl::DDL<'a>]) -> anyhow::Result<()>;
}
