use syn::parse::Parse;

use super::kw;
use crate::variant::*;

pub struct Truncate {
    pub table_name: Variant,
}

impl Parse for Truncate {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: kw::TRUNCATE = input.parse()?;

        let _: kw::TABLE = input.parse()?;

        let table_name: Variant = input.parse()?;

        Ok(Truncate { table_name })
    }
}
