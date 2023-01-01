use quote::quote;

use linq_sql_parser::{NamedColumn, Select, SelectColumns};

use crate::gen::CodeGen;

impl CodeGen for Select {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let cols = self.cols.gen_ir_code()?;

        let from = self.from.gen_ir_code()?;

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
            ::linq_rs::dml::Selecter {
                cols: #cols,
                from: #from,
                cond: #cond,
                limit: #limit,
                order_by: #order,
            }
        })
    }
}

impl CodeGen for SelectColumns {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        match self {
            Self::All => Ok(quote! {
                ::linq_rs::dml::SelectColumns::All
            }),
            Self::Expr(expr) => Ok(quote!(#expr.into())),

            Self::NamedColumns(cols) => {
                let mut token_streams = vec![];

                for col in cols {
                    token_streams.push(col.gen_ir_code()?);
                }

                Ok(quote! {
                    ::linq_rs::dml::SelectColumns::NamedColumns(vec![#(#token_streams,)*])
                })
            }
        }
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
            ::linq_rs::dml::SelectNamedColumn {
                name: #name,
                aliase: #aliase
            }
        })
    }
}
