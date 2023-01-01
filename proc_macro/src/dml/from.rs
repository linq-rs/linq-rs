use quote::quote;

use crate::gen::CodeGen;

use linq_sql_parser::From;

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
