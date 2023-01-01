use syn::parse::Parse;

use super::{kw, CondExpr, Variant};

pub struct Delete {
    pub table_name: Variant,
    pub cond: CondExpr,
}

impl Parse for Delete {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: kw::DELETE = input.parse()?;

        let _: kw::FROM = input.parse()?;

        let table_name = input.parse()?;

        let _: kw::WHERE = input.parse()?;

        let cond = input.parse()?;

        Ok(Delete { table_name, cond })
    }
}
