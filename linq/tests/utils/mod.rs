use linq_rs::{
    driver::{InsertSupport, QueryIterator, SelectSupport},
    orm::ColumnValue,
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
    pub rows: Vec<Vec<ColumnValue>>,
    pub selecter: Vec<dml::Selecter<'a>>,
}

#[async_trait::async_trait]
impl<'a> SelectSupport<'a> for SelectDriver<'a> {
    type SelectResult = SelectResult;

    #[allow(unused)]
    async fn select(&mut self, selecter: &dml::Selecter<'a>) -> anyhow::Result<Self::SelectResult> {
        self.selecter.push(selecter.clone());

        Ok(SelectResult {
            rows: self.rows.clone().into_iter(),
            current: Default::default(),
        })
    }
}

pub struct SelectResult {
    rows: std::vec::IntoIter<Vec<ColumnValue>>,
    current: Option<Vec<ColumnValue>>,
}

#[allow(unused)]
#[async_trait::async_trait]
impl<'a> QueryIterator for SelectResult {
    async fn next(&mut self) -> anyhow::Result<bool> {
        self.current = self.rows.next();

        Ok(self.current.is_some())
    }

    /// Get column value by offset id
    async fn get(&mut self, offset: usize) -> anyhow::Result<Variant> {
        let row = self.current.as_ref().expect("Call next first");

        if row.len() < offset {
            return Err(anyhow::format_err!("Out of range"));
        }

        match &row[offset] {
            ColumnValue::Simple(_, v) => return Ok(v.clone()),
            _ => return Err(anyhow::format_err!("Not here")),
        }
    }

    /// Get column value by column name
    async fn get_by_name(&mut self, name: &str) -> anyhow::Result<Variant> {
        for col_value in self.current.as_ref().expect("Call next first") {
            match col_value {
                ColumnValue::Simple(col_name, v) if *col_name == name => return Ok(v.clone()),
                _ => continue,
            }
        }

        return Err(anyhow::format_err!("Not found col {}", name));
    }
}
