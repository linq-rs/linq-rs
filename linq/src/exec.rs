use crate::dml;

#[async_trait::async_trait]
pub trait Executor {
    type QueryResult: dml::QueryResult;
    async fn select<'a>(selecter: &dml::Selecter<'a>) -> anyhow::Result<Self::QueryResult>;
}
