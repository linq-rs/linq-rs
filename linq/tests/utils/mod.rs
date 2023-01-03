use linq_rs::{
    driver::{InsertSupport, QueryIterator, SelectSupport},
    *,
};

#[derive(Default)]
pub struct InsertDriver<'a> {
    pub values: Vec<Variant>,
    pub inserter: Vec<dml::Inserter<'a>>,
}

#[async_trait::async_trait]
impl<'a> InsertSupport<'a> for InsertDriver<'a> {
    async fn insert(
        &mut self,
        inserter: &dml::Inserter<'a>,
        values: Vec<Variant>,
    ) -> anyhow::Result<usize> {
        self.values = values;
        self.inserter.push(inserter.clone());

        Ok(1)
    }
}

#[derive(Default)]
pub struct SelectDriver<'a> {
    pub selecter: Option<dml::Selecter<'a>>,
}

#[async_trait::async_trait]
impl<'a> SelectSupport<'a> for SelectDriver<'a> {
    type SelectResult = SelectResult;

    #[allow(unused)]
    async fn select(&mut self, selecter: &dml::Selecter<'a>) -> anyhow::Result<Self::SelectResult> {
        unimplemented!()
    }
}

#[derive(Default)]
pub struct SelectResult {}

#[allow(unused)]
#[async_trait::async_trait]
impl QueryIterator for SelectResult {
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
