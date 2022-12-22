use quote::quote;
use syn::parse::Parse;

use crate::gen::CodeGen;

use super::kw;
use crate::variant::*;

pub struct Truncate {
    table_name: Variant,
}

impl Parse for Truncate {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: kw::TRUNCATE = input.parse()?;

        let _: kw::TABLE = input.parse()?;

        let table_name: Variant = input.parse()?;

        Ok(Truncate { table_name })
    }
}

impl CodeGen for Truncate {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let table_name = self.table_name.gen_ir_code()?;

        Ok(quote! {
            ::linq_rs::ddl::DDL::Truncate(#table_name)
        })
    }
}
