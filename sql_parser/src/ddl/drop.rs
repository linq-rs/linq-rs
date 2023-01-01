use proc_macro2::Ident;
use quote::quote;
use syn::parse::Parse;

use crate::gen::CodeGen;

use super::kw;

pub struct Drop {
    table_name: String,
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

impl CodeGen for Drop {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let table_name = &self.table_name;

        Ok(quote! {
            ::linq_rs::ddl::DDL::Drop(#table_name)
        })
    }
}
