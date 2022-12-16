use quote::quote;
use syn::Error;
use syn::{parse::Parse, Token};

use crate::gen::CodeGen;

use super::{keyword, Cond, Limits, Order, Variant};

pub struct SelectColumn {
    pub col_name: Variant,
    pub aliase: Option<Variant>,
}

impl Parse for SelectColumn {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let col_name: Variant = input.parse()?;

        let aliase = if input.lookahead1().peek(Token![as]) {
            let _: Token![as] = input.parse()?;

            Some(input.parse::<Variant>()?)
        } else {
            None
        };

        Ok(SelectColumn { col_name, aliase })
    }
}

impl CodeGen for SelectColumn {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let col_name = self.col_name.gen_ir_code()?;

        if let Some(aliase) = &self.aliase {
            let aliase = aliase.gen_ir_code()?;
            Ok(quote! {
                ::linq_rs_ir::SelectColumn {
                    col_name: #col_name,
                    aliase: Some(#aliase)
                }
            })
        } else {
            Ok(quote! {
                ::linq_rs_ir::SelectColumn {
                    col_name: #col_name,
                    aliase: None
                }
            })
        }
    }
}

pub struct Select {
    pub cols: Option<Vec<SelectColumn>>,
    pub cond: Option<Cond>,
    pub limits: Option<Limits>,
    pub order: Option<Order>,
}

impl Parse for Select {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: keyword::select = input.parse()?;

        let cols = if input.peek(Token![*]) {
            let _: Token![*] = input.parse()?;
            None
        } else {
            let mut cols = vec![];

            loop {
                cols.push(input.parse()?);

                if input.lookahead1().peek(Token![,]) {
                    let _: Token![,] = input.parse()?;
                    continue;
                }

                break;
            }
            Some(cols)
        };

        let mut cond = None;
        let mut limits = None;
        let mut order = None;

        while !input.is_empty() {
            let lookhead = input.lookahead1();

            if lookhead.peek(keyword::order) {
                if order.is_some() {
                    return Err(Error::new(input.span(), "order clause twice"));
                }
                order = Some(input.parse()?);
            } else if lookhead.peek(keyword::limit) {
                if limits.is_some() {
                    return Err(Error::new(input.span(), "limit clause twice"));
                }
                limits = Some(input.parse()?);
            } else if lookhead.peek(Token![where]) {
                if cond.is_some() {
                    return Err(Error::new(input.span(), "where clause twice"));
                }
                cond = Some(input.parse()?);
            } else {
                break;
            }
        }

        Ok(Select {
            cols,
            cond,
            limits,
            order,
        })
    }
}

impl CodeGen for Select {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let mut irs = vec![];

        if let Some(cols) = &self.cols {
            for col in cols {
                irs.push(col.gen_ir_code()?);
            }
        }

        Ok(quote! {
            ::linq_rs_ir::Selecter {
                cols: vec![#(#irs,)*],
                cond: None,
                order: None,
                limits: None,
            }
        })
    }
}
