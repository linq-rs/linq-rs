use linq_rs::{driver::InsertSupport, *};

#[derive(Default)]
pub struct InsertDriver<'a> {
    pub values: Vec<Variant>,
    pub inserter: Option<dml::Inserter<'a>>,
}

#[async_trait::async_trait]
impl<'a> InsertSupport<'a> for InsertDriver<'a> {
    async fn insert(
        &mut self,
        inserter: &dml::Inserter<'a>,
        values: Vec<Variant>,
    ) -> anyhow::Result<usize> {
        self.values = values;
        self.inserter = Some(inserter.clone());

        Ok(1)
    }
}
