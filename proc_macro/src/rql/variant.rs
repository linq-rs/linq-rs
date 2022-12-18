use proc_macro2::Ident;
use quote::quote;
use syn::{parse::Parse, Expr, Lit, Token};

use crate::gen::CodeGen;

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
            Ok(Variant::Expr(input.parse()?))
        } else if lookahead.peek(Lit) {
            Ok(Variant::Lit(input.parse()?))
        } else {
            Ok(Variant::Ident(input.parse()?))
        }
    }
}

impl CodeGen for Variant {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        match self {
            Variant::Ident(ident) => {
                let ident_str = format!("{}", ident);
                Ok(quote!(#ident_str))
            }
            Variant::Expr(expr) => Ok(quote!(#expr)),
            Variant::Lit(lit) => Ok(quote!(#lit)),
        }
    }
}
