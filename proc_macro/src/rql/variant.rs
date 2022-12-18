use proc_macro2::Ident;
use quote::quote;
use syn::{bracketed, parse::Parse, Expr, ExprAsync, ExprClosure, Token};

use crate::gen::CodeGen;

#[allow(dead_code)]
pub enum Variant {
    Ident(Ident),

    Sync(ExprClosure),
    Async(ExprAsync),

    Expr(Expr),
}

impl Parse for Variant {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.lookahead1().peek(Token![#]) {
            let _: Token![#] = input.parse()?;

            let content;

            bracketed!(content in input);

            let lookahead = content.lookahead1();

            if lookahead.peek(Token![||]) || lookahead.peek(Token![move]) {
                let expr: ExprClosure = content.parse()?;

                return Ok(Variant::Sync(expr));
            } else if lookahead.peek(Token![async]) {
                let expr: ExprAsync = content.parse()?;

                return Ok(Variant::Async(expr));
            } else {
                let expr: Expr = content.parse()?;

                return Ok(Variant::Expr(expr));
            }
        }

        let ident: Ident = input.parse()?;

        Ok(Variant::Ident(ident))
    }
}

impl CodeGen for Variant {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        match self {
            Self::Ident(ident) => {
                let name = ident.to_string();

                Ok(quote! {
                    ::linq_rs_ir::Variant::Constant(#name)
                })
            }
            Self::Sync(expr) => Ok(quote! {
                ::linq_rs_ir::Variant::Eval((#expr)())
            }),
            Self::Async(expr) => Ok(quote! {
                ::linq_rs_ir::Variant::Eval((#expr).await)
            }),
            Self::Expr(expr) => Ok(quote! {
                ::linq_rs_ir::Variant::Constant(#expr)
            }),
        }
    }
}
