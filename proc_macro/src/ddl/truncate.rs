use quote::quote;

use crate::gen::CodeGen;

use linq_sql_parser::Truncate;

impl CodeGen for Truncate {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let table_name = self.table_name.gen_ir_code()?;

        Ok(quote! {
            ::linq_rs::ddl::DDL::Truncate(#table_name)
        })
    }
}
