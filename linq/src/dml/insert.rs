use super::Columns;

#[derive(Debug, Clone, PartialEq)]
pub struct Inserter<'a> {
    pub table_name: &'a str,
    pub cols: Columns<'a>,
}
