use quote::quote;
use syn::{parenthesized, parse::Parse, Expr, Token};

use crate::gen::CodeGen;

use super::{cond, Limit, OrderBy, Variant};

pub struct Select {
    cols: SelectColumns,
    cond: Option<cond::CondExpr>,
    limit: Option<Limit>,
    order: Option<OrderBy>,
}

impl Select {
    pub fn new(
        cols: SelectColumns,
        cond: Option<cond::CondExpr>,
        limit: Option<Limit>,
        order: Option<OrderBy>,
    ) -> Self {
        Self {
            cols,
            cond,
            limit,
            order,
        }
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

        let limit = if let Some(limit) = &self.limit {
            let token_stream = limit.gen_ir_code()?;
            quote!(Some(#token_stream))
        } else {
            quote!(None)
        };

        let order = if let Some(order) = &self.order {
            let token_stream = order.gen_ir_code()?;
            quote!(Some(#token_stream))
        } else {
            quote!(None)
        };

        Ok(quote! {
            ::linq_rs_ir::Selecter {
                cols: #cols,
                cond: #cond,
                limit: #limit,
                order_by: #order,
            }
        })
    }
}

pub enum SelectColumns {
    All,
    NamedColumns(Vec<NamedColumn>),
    Expr(Expr),
}

impl Parse for SelectColumns {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![*]) {
            let _: Token![*] = input.parse()?;
            return Ok(SelectColumns::All);
        } else if lookahead.peek(Token![#]) {
            let _: Token![#] = input.parse()?;
            let content;
            parenthesized!(content in input);

            let expr: Expr = content.parse()?;

            let _: Token![*] = input.parse()?;

            return Ok(SelectColumns::Expr(expr));
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

            return Ok(SelectColumns::NamedColumns(cols));
        }
    }
}

impl CodeGen for SelectColumns {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        match self {
            Self::All => Ok(quote! {
                ::linq_rs_ir::SelectColumns::All
            }),
            Self::Expr(expr) => Ok(quote!(#expr.into())),

            Self::NamedColumns(cols) => {
                let mut token_streams = vec![];

                for col in cols {
                    token_streams.push(col.gen_ir_code()?);
                }

                Ok(quote! {
                    ::linq_rs_ir::SelectColumns::NamedColumns(vec![#(#token_streams,)*])
                })
            }
        }
    }
}

pub struct NamedColumn {
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
            ::linq_rs_ir::SelectNamedColumn {
                name: #name,
                aliase: #aliase
            }
        })
    }
}
