use proc_macro2::Ident;
use syn::{parse::Parse, Expr, Lit, Token};

pub enum Variant {
    Ident(Ident),
    Lit(Lit),
    Expr(Expr),
}

impl Parse for Variant {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Token![#]) {
            let _: Token!(#) = input.parse()?;
            // let content;
            // bracketed!(content in input);
            Ok(Variant::Expr(input.parse()?))
        } else if lookahead.peek(Lit) {
            Ok(Variant::Lit(input.parse()?))
        } else {
            Ok(Variant::Ident(input.parse()?))
        }
    }
}
