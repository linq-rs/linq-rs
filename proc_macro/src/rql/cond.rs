use quote::quote;
use syn::{parenthesized, parse::Parse, token::Paren, Token};

use crate::gen::CodeGen;

use super::{kw, Variant};

enum Op {
    NotEq(Token!(!=)),
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
        } else if lookahead.peek(kw::like) {
            Ok(Op::Like(input.parse()?))
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
            Self::NotEq(_) => Ok(quote!(::linq_rs::CondOp::NotEq)),
            Self::Eq(_) => Ok(quote!(::linq_rs::CondOp::Eq)),
            Self::Gt(_) => Ok(quote!(::linq_rs::CondOp::Gt)),
            Self::Lt(_) => Ok(quote!(::linq_rs::CondOp::Lt)),
            Self::Gte(_) => Ok(quote!(::linq_rs::CondOp::Gte)),
            Self::Lte(_) => Ok(quote!(::linq_rs::CondOp::Lte)),
            Self::Like(_) => Ok(quote!(::linq_rs::CondOp::Like)),
            Self::In(_) => Ok(quote!(::linq_rs::CondOp::In)),
            Self::And(_) => Ok(quote!(::linq_rs::CondOp::And)),
            Self::Or(_) => Ok(quote!(::linq_rs::CondOp::Or)),
        }
    }
}

enum CondParameter {
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

impl CodeGen for CondParameter {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        match self {
            CondParameter::CondExpr(expr) => {
                let expr = expr.gen_ir_code()?;

                Ok(quote! {
                    ::linq_rs::CondParam::CondExpr(Box::new(#expr ))
                })
            }
            CondParameter::Variant(v) => match v {
                Variant::Expr(expr) => Ok(quote! {
                    ::linq_rs::CondParam::Variant(#expr.into())
                }),
                Variant::Ident(ident) => {
                    let v = format!("{}", ident);
                    Ok(quote! {
                        ::linq_rs::CondParam::Variant(#v.into())
                    })
                }
                Variant::Lit(lit) => Ok(quote! {
                    ::linq_rs::CondParam::Variant(#lit.into())
                }),
            },
            CondParameter::VariantList(variants) => {
                let mut token_streams = vec![];

                for v in variants {
                    let stream = match v {
                        Variant::Expr(expr) => quote! {
                            #expr.into()
                        },
                        Variant::Ident(ident) => {
                            let v = format!("{}", ident);
                            quote! {
                                #v.into()
                            }
                        }
                        Variant::Lit(lit) => quote! {
                            #lit.into()
                        },
                    };

                    token_streams.push(stream);
                }

                Ok(quote! {
                    ::linq_rs::CondParam::VariantList(vec![#(#token_streams,)*])
                })
            }
        }
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
        let op_stream = self.op.gen_ir_code()?;
        let lhs_stream = self.lhs.gen_ir_code()?;
        let rhs_stream = self.rhs.gen_ir_code()?;

        Ok(quote! {
            ::linq_rs::CondExpr {
                op: #op_stream,
                lhs:#lhs_stream,
                rhs:#rhs_stream
            }
        })
    }
}
