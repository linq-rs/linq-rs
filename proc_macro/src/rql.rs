mod select;
pub use select::*;

mod kw;
pub use kw::*;

mod variant;
pub use variant::*;

mod cond;
pub use cond::*;

mod limit;
pub use limit::*;

mod order;
pub use order::*;

use syn::{parse::Parse, Token};

use crate::CodeGen;

pub enum RQL {
    Select(Select),
}

impl Parse for RQL {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut cond = None;
        let mut limit = None;
        let mut order = None;

        loop {
            let lookahead = input.lookahead1();

            if lookahead.peek(Token!(where)) {
                if cond.is_some() {
                    return Err(syn::Error::new(input.span(), "twice where clause"));
                }

                let _: Token!(where) = input.parse()?;

                cond = Some(input.parse()?);
            } else if lookahead.peek(kw::limit) {
                if limit.is_some() {
                    return Err(syn::Error::new(input.span(), "twice limit clause"));
                }

                limit = Some(input.parse()?);
            } else if lookahead.peek(kw::order) {
                if order.is_some() {
                    return Err(syn::Error::new(input.span(), "twice order clause"));
                }

                order = Some(input.parse()?);
            } else if lookahead.peek(kw::select) {
                let _: kw::select = input.parse()?;

                let cols = input.parse()?;

                return Ok(RQL::Select(Select::new(cols, cond, limit, order)));
            } else {
                return Err(syn::Error::new(
                    input.span(),
                    "Expect where/order/limit/... keyword",
                ));
            }
        }
    }
}

impl CodeGen for RQL {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        match self {
            Self::Select(select) => return select.gen_ir_code(),
        }
    }
}
