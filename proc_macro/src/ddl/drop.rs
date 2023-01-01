use quote::quote;

use crate::gen::CodeGen;

use linq_sql_parser::Drop;

impl CodeGen for Drop {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let table_name = &self.table_name;

        Ok(quote! {
            ::linq_rs::ddl::DDL::Drop(#table_name)
        })
    }
}
