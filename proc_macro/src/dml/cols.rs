use quote::quote;
use syn::{parse::Parse, Expr, Token};

use crate::gen::CodeGen;

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

impl CodeGen for Columns {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        match self {
            Self::NamedColumns(cols) => {
                let mut col_streams = vec![];

                for col in cols {
                    col_streams.push(col.gen_ir_code()?);
                }

                Ok(quote! {
                    vec![#(#col_streams,)*].into()
                })
            }
            Self::Expr(expr) => Ok(quote!(#expr.into())),
        }
    }
}
