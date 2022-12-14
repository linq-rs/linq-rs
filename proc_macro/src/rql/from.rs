use proc_macro2::Ident;
use syn::parse::Parse;

use super::keyword;

#[derive(Debug, Clone, PartialEq)]
pub struct From {
    table_name: Ident,
}

impl Parse for From {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: keyword::from = input.parse()?;

        let table_name: Ident = input.parse()?;

        // TODO: JOIN Grammar support

        Ok(From { table_name })
    }
}
