#[derive(Debug, Clone, PartialEq)]
pub struct Inserter<'a> {
    pub table_name: &'a str,
    pub cols: Vec<&'a str>,
}
