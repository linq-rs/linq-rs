#[derive(Debug, Clone, PartialEq)]
pub struct Columns<'a>(Vec<&'a str>);

impl<'a, 'b> From<&'b [&'a str]> for Columns<'a> {
    fn from(v: &'b [&'a str]) -> Self {
        Self(v.to_vec())
    }
}

impl<'a, 'b, const N: usize> From<&'b [&'a str; N]> for Columns<'a> {
    fn from(v: &'b [&'a str; N]) -> Self {
        Self(v.to_vec())
    }
}

impl<'a> From<Vec<&'a str>> for Columns<'a> {
    fn from(v: Vec<&'a str>) -> Self {
        Self(v)
    }
}
