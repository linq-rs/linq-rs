#[derive(Debug, Clone, PartialEq)]
pub struct OrderBy<'a> {
    pub col_name: &'a str,
    pub desc: bool,
}
