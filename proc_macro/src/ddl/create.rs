use quote::quote;

use crate::gen::CodeGen;

use linq_sql_parser::Create;

impl CodeGen for Create {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let table_name = self.table_name.gen_ir_code()?;

        let mut cols = vec![];

        for col in &self.cols {
            cols.push(col.gen_ir_code()?);
        }

        let mut constraints = vec![];

        for c in &self.constraints {
            constraints.push(c.gen_ir_code()?);
        }

        Ok(quote! {
            ::linq_rs::ddl::DDL::Create(::linq_rs::ddl::Create {
                table_name: #table_name,
                cols: vec![#(#cols,)*],
                constraints: vec![#(#constraints,)*],
            })
        })
    }
}
