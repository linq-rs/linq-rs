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

use syn::parse::Parse;

use crate::CodeGen;

pub enum RQL {
    Select(Select),
}

impl Parse for RQL {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(select) {
            return Ok(RQL::Select(input.parse::<Select>()?));
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
