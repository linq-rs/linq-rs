use proc_macro2::Ident;
use quote::ToTokens;
use syn::{parse::Parse, ExprClosure, Token};

#[allow(dead_code)]
pub enum Variant {
    Ident(Ident),

    Closure(ExprClosure),
}

impl Parse for Variant {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.lookahead1().peek(Token![||]) {
            let expr: ExprClosure = input.parse()?;

            eprintln!("{}", expr.to_token_stream().to_string());

            return Ok(Variant::Closure(expr));
        }

        let ident: Ident = input.parse()?;

        Ok(Variant::Ident(ident))
    }
}
