#[derive(Debug, Clone, PartialEq)]
pub struct Limit {
    pub count: usize,
    pub offset: Option<usize>,
}
