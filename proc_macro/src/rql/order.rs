use syn::parse::Parse;

use super::{keyword, Variant};

pub struct Order {
    pub col: Variant,
}

impl Parse for Order {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: keyword::order = input.parse()?;
        let _: keyword::by = input.parse()?;

        let col: Variant = input.parse()?;

        Ok(Order { col })
    }
}
