use syn::parse::Parse;

use super::{kw, Columns, CondExpr, Variant};

pub struct Update {
    pub table_name: Variant,
    pub cols: Columns,
    pub cond: CondExpr,
}

impl Parse for Update {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: kw::UPDATE = input.parse()?;

        let table_name = input.parse()?;

        let cols = input.parse()?;

        let _: kw::WHERE = input.parse()?;

        let cond = input.parse()?;

        Ok(Update {
            table_name,
            cols,
            cond,
        })
    }
}
