use syn::{parse::Parse, Token};

use super::{kw, Variant};

pub struct OrderBy {
    pub name: Variant,
    pub order: Order,
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

pub enum Order {
    ASC,
    DESC,
    Variant(Variant),
}
