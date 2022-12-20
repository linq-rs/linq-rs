use quote::quote;
use syn::{parenthesized, parse::Parse, Expr, Token};

use crate::gen::CodeGen;

use super::{cond, kw, Variant};

pub struct Select {
    cols: Columns,
    cond: Option<cond::CondExpr>,
}

impl Parse for Select {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: kw::select = input.parse()?;

        let cols: Columns = input.parse()?;

        let cond = if input.lookahead1().peek(Token!(where)) {
            let _: Token!(where) = input.parse()?;

            Some(input.parse()?)
        } else {
            None
        };

        Ok(Select { cols, cond })
    }
}

impl CodeGen for Select {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let cols = self.cols.gen_ir_code()?;

        let cond = if let Some(cond) = &self.cond {
            let token_stream = cond.gen_ir_code()?;
            quote!(Some(#token_stream))
        } else {
            quote!(None)
        };

        Ok(quote! {
            ::linq_rs_ir::dml::Selecter {
                cols: #cols,
                cond: #cond,
            }
        })
    }
}

enum Columns {
    All,
    NamedColumns(Vec<NamedColumn>),
    Expr(Expr),
}

impl Parse for Columns {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![*]) {
            let _: Token![*] = input.parse()?;
            return Ok(Columns::All);
        } else if lookahead.peek(Token![#]) {
            let _: Token![#] = input.parse()?;
            let content;
            parenthesized!(content in input);

            let expr: Expr = content.parse()?;

            let _: Token![*] = input.parse()?;

            return Ok(Columns::Expr(expr));
        } else {
            let mut cols = vec![];
            loop {
                let col: NamedColumn = input.parse()?;

                cols.push(col);

                if input.lookahead1().peek(Token![,]) {
                    let _: Token![,] = input.parse()?;
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
            Self::All => Ok(quote! {
                ::linq_rs_ir::dml::SelectColumns::All
            }),
            Self::Expr(expr) => Ok(quote!(#expr.into())),

            Self::NamedColumns(cols) => {
                let mut token_streams = vec![];

                for col in cols {
                    token_streams.push(col.gen_ir_code()?);
                }

                Ok(quote! {
                    ::linq_rs_ir::dml::SelectColumns::NamedColumns(vec![#(#token_streams,)*])
                })
            }
        }
    }
}

struct NamedColumn {
    name: Variant,
    aliase: Option<Variant>,
}

impl Parse for NamedColumn {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Variant = input.parse()?;

        let aliase = if input.lookahead1().peek(Token![as]) {
            let _: Token![as] = input.parse()?;

            let aliase: Variant = input.parse()?;

            Some(aliase)
        } else {
            None
        };

        Ok(NamedColumn { name, aliase })
    }
}

impl CodeGen for NamedColumn {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let name = self.name.gen_ir_code()?;
        let aliase = if let Some(aliase) = &self.aliase {
            let stream = aliase.gen_ir_code()?;
            quote! {
                Some(#stream)
            }
        } else {
            quote!(None)
        };
        Ok(quote! {
            ::linq_rs_ir::dml::SelectNamedColumn {
                name: #name,
                aliase: #aliase
            }
        })
    }
}
