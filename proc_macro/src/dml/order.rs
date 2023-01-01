use quote::quote;

use crate::gen::CodeGen;

use linq_sql_parser::{Order, OrderBy};

impl CodeGen for OrderBy {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let col_name = self.name.gen_ir_code()?;

        let order = match &self.order {
            Order::ASC => {
                quote!(false)
            }
            Order::DESC => {
                quote!(true)
            }
            Order::Variant(v) => v.gen_ir_code()?,
        };

        Ok(quote! {
            ::linq_rs::dml::OrderBy {
                col_name: #col_name,
                desc: #order,
            }
        })
    }
}
