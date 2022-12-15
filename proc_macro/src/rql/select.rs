use proc_macro2::Ident;
use syn::Error;
use syn::{parse::Parse, Token};

use super::{keyword, Cond, Limits, Order};

pub struct SelectColumn {
    pub col_name: Ident,
    pub aliase: Option<Ident>,
}

impl Parse for SelectColumn {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let col_name: Ident = input.parse()?;

        let aliase = if input.lookahead1().peek(Token![as]) {
            let _: Token![as] = input.parse()?;

            Some(input.parse::<Ident>()?)
        } else {
            None
        };

        Ok(SelectColumn { col_name, aliase })
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

                if !input.lookahead1().peek(Token![,]) {
                    break;
                }
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
