use std::vec::IntoIter;

use linq_rs::{
    dml::Selecter,
    orm::{ColumnValue, Table},
    QueryIterator, SelectSupport, Variant,
};

#[derive(Default)]
pub struct AssertDriver<'a, T> {
    expect_selecter: Option<(Selecter<'a>, Box<dyn FnMut() -> Vec<T> + Sync + Send>)>,
}

impl<'a, T> AssertDriver<'a, T> {
    /// Create new [`Selecter`] assert
    pub fn exepct_selecter(
        selecter: Selecter<'a>,
        query_f: impl FnMut() -> Vec<T> + Sync + Send + 'static,
    ) -> Self {
        Self {
            expect_selecter: Some((selecter, Box::new(query_f))),
        }
    }
}

#[async_trait::async_trait]
impl<'a, T> SelectSupport<'a> for AssertDriver<'a, T>
where
    T: Send + Sync + Table,
{
    type SelectResult = AssertDriverQueryIterator<T>;
    /// Execute select stmt
    async fn select(&mut self, selecter: &Selecter<'a>) -> anyhow::Result<Self::SelectResult> {
        let (expect_selecter, mut query_f) = self.expect_selecter.take().unwrap();

        assert_eq!(selecter, &expect_selecter);

        let data = query_f();

        self.expect_selecter = Some((expect_selecter, query_f));

        Ok(AssertDriverQueryIterator {
            rows: data.into_iter(),
            current: vec![],
        })
    }
}

pub struct AssertDriverQueryIterator<T> {
    rows: IntoIter<T>,
    current: Vec<ColumnValue>,
}

#[allow(unused)]
#[async_trait::async_trait]
impl<T> QueryIterator for AssertDriverQueryIterator<T>
where
    T: Send + Sync + Table,
{
    async fn next(&mut self) -> anyhow::Result<bool> {
        match self.rows.next() {
            Some(mut t) => {
                self.current = t.into_values();
                return Ok(true);
            }
            None => return Ok(false),
        }
    }

    /// Get column value by offset id
    async fn get(&mut self, offset: usize) -> anyhow::Result<Variant> {
        unimplemented!()
    }

    /// Get column value by column name
    async fn get_by_name(&mut self, name: &str) -> anyhow::Result<Variant> {
        for col in &self.current {
            if col.col_name() == name {
                return col
                    .into_simple_value()
                    .map_err(|_| anyhow::format_err!("Col is reference table {}", col.col_name()));
            }
        }

        Err(anyhow::format_err!("Col {} not found", name))
    }
}
