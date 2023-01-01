use syn::{parenthesized, parse::Parse, Expr, Token};

use super::{cond, kw, From, Limit, OrderBy, Variant};

pub struct Select {
    pub cols: SelectColumns,
    pub from: From,
    pub cond: Option<cond::CondExpr>,
    pub limit: Option<Limit>,
    pub order: Option<OrderBy>,
}

impl Parse for Select {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: kw::SELECT = input.parse()?;

        let cols: SelectColumns = input.parse()?;

        let from: From = input.parse()?;

        let mut cond = None;
        let mut limit = None;
        let mut order = None;

        if input.lookahead1().peek(kw::WHERE) {
            let _: kw::WHERE = input.parse()?;

            cond = Some(input.parse()?);
        }

        if input.lookahead1().peek(kw::ORDER) {
            order = Some(input.parse()?);
        }

        if input.lookahead1().peek(kw::LIMIT) {
            limit = Some(input.parse()?);
        }

        return Ok(Self {
            cols,
            from,
            cond,
            limit,
            order,
        });
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

pub struct NamedColumn {
    pub name: Variant,
    pub aliase: Option<Variant>,
}

impl Parse for NamedColumn {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Variant = input.parse()?;

        let aliase = if input.lookahead1().peek(kw::AS) {
            let _: kw::AS = input.parse()?;

            let aliase: Variant = input.parse()?;

            Some(aliase)
        } else {
            None
        };

        Ok(NamedColumn { name, aliase })
    }
}
