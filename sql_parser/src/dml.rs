mod select;
pub use select::*;

mod kw;

pub use super::variant::*;

mod cond;
pub use cond::*;

mod limit;
pub use limit::*;

mod order;
pub use order::*;

mod from;
pub use from::*;

mod insert;
pub use insert::*;

mod update;
pub use update::*;

mod cols;
pub use cols::*;

mod delete;
pub use delete::*;

use syn::{parse::Parse, Token};

pub enum RQL {
    Select(Select),
    Insert(Insert),
    Update(Update),
    Delete(Delete),
}

impl Parse for RQL {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        let result = if lookahead.peek(kw::SELECT) {
            RQL::Select(input.parse()?)
        } else if lookahead.peek(kw::INSERT) {
            RQL::Insert(input.parse()?)
        } else if lookahead.peek(kw::UPDATE) {
            RQL::Update(input.parse()?)
        } else if lookahead.peek(kw::DELETE) {
            RQL::Delete(input.parse()?)
        } else {
            return Err(syn::Error::new(
                input.span(),
                "Expect select/insert/update/delete",
            ));
        };

        if input.lookahead1().peek(Token!(;)) {
            let _: Token!(;) = input.parse()?;
        }

        Ok(result)
    }
}

pub struct RQLs {
    pub rqls: Vec<RQL>,
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
