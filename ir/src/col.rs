#[derive(Debug, Clone, PartialEq)]
pub enum SelectColumns<'a> {
    // Match *
    All,

    NamedColumns(Vec<SelectNamedColumn<'a>>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectNamedColumn<'a> {
    pub name: &'a str,
    pub aliase: Option<&'a str>,
}

impl<'a> From<&'a str> for SelectNamedColumn<'a> {
    fn from(name: &'a str) -> Self {
        SelectNamedColumn { name, aliase: None }
    }
}

impl<'a> From<(&'a str, &'a str)> for SelectNamedColumn<'a> {
    fn from(pair: (&'a str, &'a str)) -> Self {
        SelectNamedColumn {
            name: pair.0,
            aliase: Some(pair.1),
        }
    }
}

impl<'a> From<Vec<&'a str>> for SelectColumns<'a> {
    fn from(cols: Vec<&'a str>) -> Self {
        SelectColumns::NamedColumns(cols.iter().map(|c| (*c).into()).collect())
    }
}

impl<'a> From<Vec<(&'a str, &'a str)>> for SelectColumns<'a> {
    fn from(cols: Vec<(&'a str, &'a str)>) -> Self {
        SelectColumns::NamedColumns(cols.iter().map(|c| (*c).into()).collect())
    }
}
