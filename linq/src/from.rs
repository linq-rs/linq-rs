#[derive(Debug, Clone, PartialEq)]
pub struct From<'a> {
    pub table_name: &'a str,
}
