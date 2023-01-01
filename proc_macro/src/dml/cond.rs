use quote::quote;

use crate::gen::CodeGen;

use linq_sql_parser::{CondExpr, CondParameter, Op, Variant};

impl CodeGen for Op {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        match self {
            Self::NotEq(_) => Ok(quote!(::linq_rs::dml::CondOp::NotEq)),
            Self::Eq(_) => Ok(quote!(::linq_rs::dml::CondOp::Eq)),
            Self::Gt(_) => Ok(quote!(::linq_rs::dml::CondOp::Gt)),
            Self::Lt(_) => Ok(quote!(::linq_rs::dml::CondOp::Lt)),
            Self::Gte(_) => Ok(quote!(::linq_rs::dml::CondOp::Gte)),
            Self::Lte(_) => Ok(quote!(::linq_rs::dml::CondOp::Lte)),
            Self::Like(_) => Ok(quote!(::linq_rs::dml::CondOp::Like)),
            Self::In(_) => Ok(quote!(::linq_rs::dml::CondOp::In)),
            Self::And(_) => Ok(quote!(::linq_rs::dml::CondOp::And)),
            Self::Or(_) => Ok(quote!(::linq_rs::dml::CondOp::Or)),
        }
    }
}

impl CodeGen for CondParameter {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        match self {
            CondParameter::CondExpr(expr) => {
                let expr = expr.gen_ir_code()?;

                Ok(quote! {
                    ::linq_rs::dml::CondParam::CondExpr(Box::new(#expr ))
                })
            }
            CondParameter::Variant(v) => match v {
                Variant::Expr(expr) => Ok(quote! {
                    ::linq_rs::dml::CondParam::Variant(#expr.into())
                }),
                Variant::Ident(ident) => {
                    let v = format!("{}", ident);
                    Ok(quote! {
                        ::linq_rs::dml::CondParam::Variant(#v.into())
                    })
                }
                Variant::Lit(lit) => Ok(quote! {
                    ::linq_rs::dml::CondParam::Variant(#lit.into())
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
                    ::linq_rs::dml::CondParam::VariantList(vec![#(#token_streams,)*])
                })
            }
        }
    }
}

impl CodeGen for CondExpr {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let op_stream = self.op.gen_ir_code()?;
        let lhs_stream = self.lhs.gen_ir_code()?;
        let rhs_stream = self.rhs.gen_ir_code()?;

        Ok(quote! {
            ::linq_rs::dml::CondExpr {
                op: #op_stream,
                lhs:#lhs_stream,
                rhs:#rhs_stream
            }
        })
    }
}
