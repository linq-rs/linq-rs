use quote::quote;
use syn::parse::Parse;

use crate::gen::CodeGen;

use super::{kw, CondExpr, Variant};

pub struct Delete {
    table_name: Variant,
    cond: CondExpr,
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

impl CodeGen for Delete {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let table_name = self.table_name.gen_ir_code()?;
        let cond = self.cond.gen_ir_code()?;

        Ok(quote! {
            ::linq_rs::dml::Deleter {
                table_name: #table_name,
                cond: #cond,
            }
        })
    }
}
