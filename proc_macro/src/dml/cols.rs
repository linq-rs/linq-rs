use quote::quote;

use crate::gen::CodeGen;

use linq_sql_parser::Columns;

impl CodeGen for Columns {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        match self {
            Self::NamedColumns(cols) => {
                let mut col_streams = vec![];

                for col in cols {
                    col_streams.push(col.gen_ir_code()?);
                }

                Ok(quote! {
                    vec![#(#col_streams,)*].into()
                })
            }
            Self::Expr(expr) => Ok(quote!(#expr.into())),
        }
    }
}
