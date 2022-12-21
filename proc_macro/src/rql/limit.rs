use quote::quote;
use syn::parse::Parse;

use crate::gen::CodeGen;

use super::{kw, Variant};

pub struct Limit {
    count: Variant,
    offset: Option<Variant>,
}

impl Parse for Limit {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: kw::limit = input.parse()?;

        let count: Variant = input.parse()?;

        let offset = if input.lookahead1().peek(kw::offset) {
            let _: kw::offset = input.parse()?;
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Limit { count, offset })
    }
}

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
