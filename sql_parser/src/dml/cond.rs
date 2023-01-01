use syn::{parenthesized, parse::Parse, token::Paren, Token};

use super::{kw, Variant};

pub enum Op {
    NotEq(Token!(!=)),
    Eq(Token!(=)),
    Gt(Token!(>)),
    Lt(Token!(<)),
    Gte(Token!(>=)),
    Lte(Token!(<=)),
    Like(kw::LIKE),
    In(Token!(in)),
    And(kw::AND),
    Or(kw::OR),
}

impl Parse for Op {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Token!(!=)) {
            Ok(Op::NotEq(input.parse()?))
        } else if lookahead.peek(Token!(>=)) {
            Ok(Op::Gte(input.parse()?))
        } else if lookahead.peek(Token!(<=)) {
            Ok(Op::Lte(input.parse()?))
        } else if lookahead.peek(Token!(=)) {
            Ok(Op::Eq(input.parse()?))
        } else if lookahead.peek(Token!(>)) {
            Ok(Op::Gt(input.parse()?))
        } else if lookahead.peek(Token!(<)) {
            Ok(Op::Lt(input.parse()?))
        } else if lookahead.peek(Token!(in)) {
            Ok(Op::In(input.parse()?))
        } else if lookahead.peek(kw::LIKE) {
            Ok(Op::Like(input.parse()?))
        } else if lookahead.peek(kw::AND) {
            Ok(Op::And(input.parse()?))
        } else if lookahead.peek(kw::OR) {
            Ok(Op::Or(input.parse()?))
        } else {
            return Err(syn::Error::new(
                input.span(),
                "Expect cond op look like <,>,in..",
            ));
        }
    }
}

pub enum CondParameter {
    VariantList(Vec<Variant>),
    Variant(Variant),
    CondExpr(Box<CondExpr>),
}

impl Parse for CondParameter {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.lookahead1().peek(Paren) {
            let mut variants = vec![];

            let content;

            parenthesized!(content in input);

            loop {
                variants.push(content.parse()?);

                if !content.lookahead1().peek(Token!(,)) {
                    break;
                }

                let _: Token!(,) = content.parse()?;
            }

            Ok(CondParameter::VariantList(variants))
        } else {
            Ok(CondParameter::Variant(input.parse()?))
        }
    }
}

pub struct CondExpr {
    pub op: Op,
    pub lhs: CondParameter,
    pub rhs: CondParameter,
}

impl Parse for CondExpr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lhs = if input.lookahead1().peek(Paren) {
            let content;

            parenthesized!(content in input);

            content.parse()?
        } else {
            let lhs: CondParameter = input.parse()?;

            let op: Op = input.parse()?;

            if let Op::In(_) = op {
                let span = input.span();

                if !input.lookahead1().peek(Paren) {
                    return Err(syn::Error::new(span, "expect ("));
                }
            }

            let rhs: CondParameter = input.parse()?;

            CondExpr { lhs, op, rhs }
        };

        let lookahead = input.lookahead1();

        if lookahead.peek(kw::AND) || lookahead.peek(kw::OR) {
            let r_op: Op = input.parse()?;
            let r_rhs: CondExpr = input.parse()?;

            return Ok(CondExpr {
                op: r_op,
                lhs: CondParameter::CondExpr(Box::new(lhs)),
                rhs: CondParameter::CondExpr(Box::new(r_rhs)),
            });
        }

        Ok(lhs)
    }
}
