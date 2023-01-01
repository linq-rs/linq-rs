use proc_macro2::Ident;
use syn::parse::Parse;

use super::kw;

pub struct Drop {
    pub table_name: String,
}

impl Parse for Drop {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: kw::DROP = input.parse()?;

        let _: kw::TABLE = input.parse()?;

        let table_name: Ident = input.parse()?;

        Ok(Drop {
            table_name: table_name.to_string(),
        })
    }
}
