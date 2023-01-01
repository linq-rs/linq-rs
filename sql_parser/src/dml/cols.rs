use syn::{parse::Parse, Expr, Token};

use super::Variant;

pub enum Columns {
    NamedColumns(Vec<Variant>),
    Expr(Expr),
}

impl Parse for Columns {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.lookahead1().peek(Token!(#)) {
            let _: Token!(#) = input.parse()?;
            let content;

            syn::parenthesized!(content in input);

            let _: Token!(*) = input.parse()?;

            return Ok(Columns::Expr(content.parse()?));
        } else {
            let content;

            syn::parenthesized!(content in input);

            let mut cols = vec![];

            loop {
                cols.push(content.parse()?);

                if content.lookahead1().peek(Token!(,)) {
                    let _: Token!(,) = content.parse()?;
                    continue;
                }

                break;
            }

            return Ok(Columns::NamedColumns(cols));
        }
    }
}
