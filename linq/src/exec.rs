use crate::{QueryResult, Selecter};

#[async_trait::async_trait]
pub trait Executor {
    type QueryResult: QueryResult;
    async fn query<'a>(selecter: &Selecter<'a>) -> anyhow::Result<Self::QueryResult>;
}
