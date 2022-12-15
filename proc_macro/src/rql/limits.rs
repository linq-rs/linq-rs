use syn::{parse::Parse, LitInt};

use super::keyword;

pub struct Limits {
    pub count: LitInt,
    pub offset: Option<LitInt>,
}

impl Parse for Limits {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: keyword::limit = input.parse()?;

        // limit return rows count
        let count: LitInt = input.parse()?;

        let mut offset = None;

        if input.lookahead1().peek(keyword::offset) {
            let _: keyword::offset = input.parse()?;
            offset = Some(input.parse()?);
        }

        Ok(Limits { count, offset })
    }
}
