use linq_rs::dml::*;
use linq_rs::orm::*;
use linq_rs::*;

struct NullDriver {}

struct NullQueryIterator {}

#[async_trait::async_trait]
impl QueryIterator for NullQueryIterator {
    async fn next(&mut self) -> anyhow::Result<bool> {
        unimplemented!()
    }

    /// Get column value by offset id
    async fn get(&mut self, offset: usize) -> anyhow::Result<Variant> {
        unimplemented!()
    }

    /// Get column value by column name
    async fn get_by_name(&mut self, name: &str) -> anyhow::Result<Variant> {
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl SelectSupport for NullDriver {
    type SelectResult = NullQueryIterator;
    /// Execute select stmt
    async fn select<'a>(&mut self, selecter: &Selecter<'a>) -> anyhow::Result<Self::SelectResult> {
        unimplemented!()
    }
}

struct NullTable {}

impl Table for NullTable {
    fn table_name(&self) -> &'static str {
        return "null_table";
    }

    fn cols(&self) -> &'static [&'static Column] {
        static cols: &'static [&'static Column] = [].as_slice();

        cols
    }

    fn write(&mut self, values: Vec<ColumnValue>) -> anyhow::Result<()> {
        unimplemented!()
    }

    fn read(&self) -> anyhow::Result<Vec<ColumnValue>> {
        unimplemented!()
    }
}

#[async_std::test]
async fn test_select() -> anyhow::Result<()> {
    let mut driver = NullDriver {};
    let a = 1;

    let t: NullTable = rql_where! (id = 1 AND c = #a).select(&mut driver).await?;

    Ok(())
}
