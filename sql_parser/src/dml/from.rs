use quote::quote;
use syn::parse::Parse;

use crate::gen::CodeGen;

use super::{kw, Variant};

pub struct From {
    table_name: Variant,
}

impl Parse for From {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: kw::FROM = input.parse()?;

        let table_name: Variant = input.parse()?;

        Ok(From { table_name })
    }
}

impl CodeGen for From {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let table_name = self.table_name.gen_ir_code()?;

        Ok(quote! {
            ::linq_rs::dml::SelectFrom {
                table_name: #table_name,
            }
        })
    }
}