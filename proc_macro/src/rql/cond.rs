use quote::quote;
use syn::{parenthesized, parse::Parse, token::Paren, Token};

use crate::gen::CodeGen;

use super::{kw, Variant};

enum Op {
    Eq(Token!(=)),
    Gt(Token!(>)),
    Lt(Token!(<)),
    Gte(Token!(>=)),
    Lte(Token!(<=)),
    Like(kw::like),
    In(Token!(in)),
    And(kw::and),
    Or(kw::or),
}

impl Parse for Op {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Token!(=)) {
            Ok(Op::Eq(input.parse()?))
        } else if lookahead.peek(Token!(>)) {
            Ok(Op::Gt(input.parse()?))
        } else if lookahead.peek(Token!(<)) {
            Ok(Op::Lt(input.parse()?))
        } else if lookahead.peek(Token!(>=)) {
            Ok(Op::Gte(input.parse()?))
        } else if lookahead.peek(Token!(<=)) {
            Ok(Op::Lte(input.parse()?))
        } else if lookahead.peek(Token!(in)) {
            Ok(Op::Like(input.parse()?))
        } else if lookahead.peek(kw::like) {
            Ok(Op::In(input.parse()?))
        } else if lookahead.peek(kw::and) {
            Ok(Op::And(input.parse()?))
        } else if lookahead.peek(kw::or) {
            Ok(Op::Or(input.parse()?))
        } else {
            return Err(syn::Error::new(
                input.span(),
                "Expect cond op look like <,>,in..",
            ));
        }
    }
}

impl CodeGen for Op {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        match self {
            Self::Eq(_) => Ok(quote!(::linq_rs_ir::dml::CondOp::Eq)),
            Self::Gt(_) => Ok(quote!(::linq_rs_ir::dml::CondOp::Gt)),
            Self::Lt(_) => Ok(quote!(::linq_rs_ir::dml::CondOp::Lt)),
            Self::Gte(_) => Ok(quote!(::linq_rs_ir::dml::CondOp::Gte)),
            Self::Lte(_) => Ok(quote!(::linq_rs_ir::dml::CondOp::Lte)),
            Self::Like(_) => Ok(quote!(::linq_rs_ir::dml::CondOp::Like)),
            Self::In(_) => Ok(quote!(::linq_rs_ir::dml::CondOp::In)),
            Self::And(_) => Ok(quote!(::linq_rs_ir::dml::CondOp::And)),
            Self::Or(_) => Ok(quote!(::linq_rs_ir::dml::CondOp::Or)),
        }
    }
}

enum CondParameter {
    Variant(Variant),
    CondExpr(Box<CondExpr>),
}

impl Parse for CondParameter {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(CondParameter::Variant(input.parse()?))
    }
}

impl CodeGen for CondParameter {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        unimplemented!()
    }
}

pub struct CondExpr {
    op: Op,
    lhs: CondParameter,
    rhs: CondParameter,
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

            let rhs: CondParameter = input.parse()?;

            CondExpr { lhs, op, rhs }
        };

        let lookahead = input.lookahead1();

        if lookahead.peek(kw::and) || lookahead.peek(kw::or) {
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

impl CodeGen for CondExpr {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        unimplemented!()
    }
}
