mod select;
pub use select::*;

mod keyword;
pub use keyword::*;

mod from;
pub use from::*;

mod cond;
pub use cond::*;

mod limits;
pub use limits::*;

mod order;
pub use order::*;

mod value;
pub use value::*;

use syn::parse::Parse;

pub enum RQL {
    Select(Select),
}

impl Parse for RQL {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(keyword::select) {
            return Ok(RQL::Select(input.parse::<Select>()?));
        }

        unimplemented!()
    }
}
