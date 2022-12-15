use proc_macro2::Ident;
use syn::{parenthesized, parse::Parse, token::Paren, Lit, Token};

use super::keyword;

pub enum CondOp {
    Eq(Token![=]),
    Gt(Token![>]),
    Lt(Token![<]),
    Gte(Token![>=]),
    Lte(Token![<=]),
    Like(keyword::like),
    In(Token![in]),
    And(keyword::and),
    Or(keyword::or),
}

impl Parse for CondOp {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Token![=]) {
            return Ok(CondOp::Eq(input.parse()?));
        } else if lookahead.peek(Token![<]) {
            return Ok(CondOp::Lt(input.parse()?));
        } else if lookahead.peek(Token![>]) {
            return Ok(CondOp::Gt(input.parse()?));
        } else if lookahead.peek(Token![<=]) {
            return Ok(CondOp::Lte(input.parse()?));
        } else if lookahead.peek(Token![>=]) {
            return Ok(CondOp::Gte(input.parse()?));
        } else if lookahead.peek(keyword::like) {
            return Ok(CondOp::Like(input.parse()?));
        } else if lookahead.peek(Token![in]) {
            return Ok(CondOp::In(input.parse()?));
        } else if lookahead.peek(keyword::and) {
            return Ok(CondOp::And(input.parse()?));
        } else if lookahead.peek(keyword::or) {
            return Ok(CondOp::Or(input.parse()?));
        }

        return Err(syn::Error::new(input.span(), "Expect cond ops(=,>,< ..)"));
    }
}

pub enum CondValue {
    Ident(Ident),
    Lit(Lit),
    CondExpr(Box<CondExpr>),
}

impl Parse for CondValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(syn::Ident) {
            return Ok(CondValue::Ident(input.parse()?));
        } else if lookahead.peek(Lit) {
            return Ok(CondValue::Lit(input.parse()?));
        } else {
            return Ok(CondValue::CondExpr(Box::new(input.parse()?)));
        }
    }
}

pub struct CondExpr {
    pub op: CondOp,
    pub lhs: CondValue,
    pub rhs: CondValue,
}

impl Parse for CondExpr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lhs = if input.lookahead1().peek(Paren) {
            let content;

            parenthesized!(content in input);

            content.parse()?
        } else {
            let lhs: CondValue = input.parse()?;

            let op: CondOp = input.parse()?;

            let rhs: CondValue = input.parse()?;

            CondExpr { lhs, op, rhs }
        };

        let lookahead = input.lookahead1();

        if lookahead.peek(keyword::and) || lookahead.peek(keyword::or) {
            let r_op: CondOp = input.parse()?;
            let r_rhs: CondExpr = input.parse()?;

            return Ok(CondExpr {
                op: r_op,
                lhs: CondValue::CondExpr(Box::new(lhs)),
                rhs: CondValue::CondExpr(Box::new(r_rhs)),
            });
        }

        Ok(lhs)
    }
}

pub struct Cond {
    pub expr: CondExpr,
}

impl Parse for Cond {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: Token![where] = input.parse()?;

        let expr: CondExpr = input.parse()?;

        Ok(Cond { expr })
    }
}
