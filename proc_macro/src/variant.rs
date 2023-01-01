use quote::quote;

use crate::gen::CodeGen;
use linq_sql_parser::Variant;

impl CodeGen for Variant {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        match self {
            Variant::Ident(ident) => {
                let ident_str = format!("{}", ident);
                Ok(quote!(#ident_str))
            }
            Variant::Expr(expr) => Ok(quote!(#expr)),
            Variant::Lit(lit) => Ok(quote!(#lit)),
        }
    }
}
