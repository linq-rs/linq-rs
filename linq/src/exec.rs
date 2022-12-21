use crate::dml;
use crate::Variant;

#[async_trait::async_trait]
pub trait QueryResult {
    async fn next(&mut self) -> anyhow::Result<bool>;

    /// Get column value by offset id
    async fn get(&mut self, offset: usize) -> anyhow::Result<Variant>;

    /// Get column value by column name
    async fn get_by_name(&mut self, name: &str) -> anyhow::Result<Variant>;
}

#[async_trait::async_trait]
pub trait Executor {
    type QueryResult: QueryResult;
    async fn select<'a>(selecter: &dml::Selecter<'a>) -> anyhow::Result<Self::QueryResult>;

    async fn update<'a>(updater: &dml::Updater<'a>, values: &[Variant]) -> anyhow::Result<usize>;

    async fn insert<'a>(inserter: &dml::Inserter<'a>, values: &[Variant]) -> anyhow::Result<usize>;

    async fn delete<'a>(inserter: &dml::Deleter<'a>) -> anyhow::Result<usize>;
}
