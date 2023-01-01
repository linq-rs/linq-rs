use quote::quote;
use syn::{parse::Parse, Token};

use crate::gen::CodeGen;

use super::{kw, Variant};

pub struct OrderBy {
    name: Variant,
    order: Order,
}

impl Parse for OrderBy {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: kw::ORDER = input.parse()?;
        let _: kw::BY = input.parse()?;

        let name: Variant = input.parse()?;

        let order = if input.lookahead1().peek(kw::ASC) {
            let _: kw::ASC = input.parse()?;
            Order::ASC
        } else if input.lookahead1().peek(kw::DESC) {
            let _: kw::DESC = input.parse()?;
            Order::DESC
        } else if input.lookahead1().peek(Token![#]) {
            Order::Variant(input.parse()?)
        } else {
            Order::ASC
        };

        Ok(OrderBy { name, order })
    }
}

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

enum Order {
    ASC,
    DESC,
    Variant(Variant),
}
