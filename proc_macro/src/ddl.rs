use quote::quote;

use crate::gen::CodeGen;

mod alter;
mod cols;
mod constraint;
mod create;
mod drop;
mod truncate;

use linq_sql_parser::{DDLs, DDL};

impl CodeGen for DDL {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        match self {
            Self::Create(create) => return create.gen_ir_code(),
            Self::Alter(alter) => return alter.gen_ir_code(),
            Self::Drop(drop) => return drop.gen_ir_code(),
            Self::Truncate(truncate) => return truncate.gen_ir_code(),
        }
    }
}

impl CodeGen for DDLs {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let mut ddls = vec![];

        for ddl in &self.ddls {
            ddls.push(ddl.gen_ir_code()?);
        }

        Ok(quote! {
            vec![#(#ddls,)*]
        })
    }
}
