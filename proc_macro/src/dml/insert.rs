use quote::quote;

use crate::gen::CodeGen;

use linq_sql_parser::Insert;

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
