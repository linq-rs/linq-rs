use quote::quote;
use syn::parse::Parse;

use crate::gen::CodeGen;

use super::{kw, Columns, Variant};

pub struct Insert {
    table_name: Variant,
    cols: Columns,
}

impl Parse for Insert {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: kw::insert = input.parse()?;

        let _: kw::into = input.parse()?;

        let table_name: Variant = input.parse()?;

        let cols = input.parse()?;

        Ok(Insert { table_name, cols })
    }
}

impl CodeGen for Insert {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let table_name = self.table_name.gen_ir_code()?;

        let cols = self.cols.gen_ir_code()?;

        Ok(quote! {
            ::linq_rs::dml::Inserter {
                table_name: #table_name,
                cols: #cols,
            }
        })
    }
}
