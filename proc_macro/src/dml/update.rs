use quote::quote;
use syn::parse::Parse;

use crate::gen::CodeGen;

use super::{kw, Columns, CondExpr, Variant};

pub struct Update {
    table_name: Variant,
    cols: Columns,
    cond: CondExpr,
}

impl Parse for Update {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: kw::UPDATE = input.parse()?;

        let table_name = input.parse()?;

        let cols = input.parse()?;

        let _: kw::WHERE = input.parse()?;

        let cond = input.parse()?;

        Ok(Update {
            table_name,
            cols,
            cond,
        })
    }
}

impl CodeGen for Update {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let table_name = self.table_name.gen_ir_code()?;
        let cols = self.cols.gen_ir_code()?;
        let cond = self.cond.gen_ir_code()?;

        Ok(quote! {
            ::linq_rs::dml::Updater {
                table_name: #table_name,
                cols: #cols,
                cond: #cond,
            }
        })
    }
}
