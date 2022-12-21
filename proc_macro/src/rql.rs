mod select;
use quote::quote;
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

mod from;
pub use from::*;

use syn::parse::Parse;

use crate::CodeGen;

pub enum RQL {
    Select(Select),
}

impl Parse for RQL {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::select) {
            return Ok(RQL::Select(input.parse()?));
        }

        unimplemented!()
    }
}

impl CodeGen for RQL {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        match self {
            Self::Select(select) => return select.gen_ir_code(),
        }
    }
}

pub struct RQLs {
    rqls: Vec<RQL>,
}

impl Parse for RQLs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut rqls = vec![];

        while !input.is_empty() {
            rqls.push(input.parse()?);
        }

        Ok(RQLs { rqls })
    }
}

impl CodeGen for RQLs {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let mut rqls = vec![];

        for rql in &self.rqls {
            rqls.push(rql.gen_ir_code()?);
        }

        Ok(quote! {
            (#(#rqls,)*)
        })
    }
}
