use syn::parse::Parse;

use super::{kw, Variant};

pub struct Limit {
    pub count: Variant,
    pub offset: Option<Variant>,
}

impl Parse for Limit {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: kw::LIMIT = input.parse()?;

        let count: Variant = input.parse()?;

        let offset = if input.lookahead1().peek(kw::OFFSET) {
            let _: kw::OFFSET = input.parse()?;
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Limit { count, offset })
    }
}
