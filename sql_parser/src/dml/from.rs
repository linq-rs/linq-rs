use syn::parse::Parse;

use super::{kw, Variant};

pub struct From {
    pub table_name: Variant,
}

impl Parse for From {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: kw::FROM = input.parse()?;

        let table_name: Variant = input.parse()?;

        Ok(From { table_name })
    }
}
