use syn::{parse::Parse, Token};

mod alter;
pub use alter::*;
mod cols;
pub use cols::*;
mod constraint;
pub use constraint::*;
mod create;
pub use create::*;
mod drop;
pub use drop::*;
mod kw;
mod truncate;
pub use truncate::*;

pub enum DDL {
    Create(Create),
    Alter(Alter),
    Drop(Drop),
    Truncate(Truncate),
}

impl Parse for DDL {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        let ddl = if lookahead.peek(kw::CREATE) {
            DDL::Create(input.parse()?)
        } else if lookahead.peek(kw::ALTER) {
            DDL::Alter(input.parse()?)
        } else if lookahead.peek(kw::DROP) {
            DDL::Drop(input.parse()?)
        } else if lookahead.peek(kw::TRUNCATE) {
            DDL::Truncate(input.parse()?)
        } else {
            return Err(syn::Error::new(
                input.span(),
                "Expect CREATE/ALTER/DROP/TRUNCATE ",
            ));
        };

        if input.lookahead1().peek(Token!(;)) {
            let _: Token!(;) = input.parse()?;
        }

        Ok(ddl)
    }
}

pub struct DDLs {
    pub ddls: Vec<DDL>,
}

impl Parse for DDLs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut ddls = vec![];

        while !input.is_empty() {
            ddls.push(input.parse()?);
        }

        Ok(DDLs { ddls })
    }
}
