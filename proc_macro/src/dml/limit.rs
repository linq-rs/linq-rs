use quote::quote;

use crate::gen::CodeGen;

use linq_sql_parser::Limit;

impl CodeGen for Limit {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let offset_stream = if let Some(offset) = &self.offset {
            let stream = offset.gen_ir_code()?;

            quote!(Some(#stream))
        } else {
            quote!(None)
        };

        let count = &self.count.gen_ir_code()?;

        Ok(quote! {
            ::linq_rs::dml::Limit {
                count: #count,
                offset: #offset_stream,
            }
        })
    }
}
