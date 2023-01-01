use quote::quote;

use crate::gen::CodeGen;

use linq_sql_parser::{Column, IrType};

impl CodeGen for Column {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let name = &self.name;
        let col_type = self.col_type.gen_ir_code()?;
        let not_null = self.not_null;

        let default_value = if let Some(default_value) = &self.default_value {
            let default_value = default_value.gen_ir_code()?;
            quote!(Some(#default_value))
        } else {
            quote!(None)
        };

        let primary = if let Some(primary) = &self.primary {
            quote!(Some(#primary))
        } else {
            quote!(None)
        };

        Ok(quote! {
            ::linq_rs::ddl::Column {
                name: #name,
                col_type: #col_type,
                not_null: #not_null,
                default_value: #default_value,
                primary: #primary,
            }
        })
    }
}

impl CodeGen for IrType {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let name = match self {
            Self::Int(_) => quote!(Int),
            Self::Float(_) => quote!(Float),
            Self::String(_) => quote!(String),
            Self::Bytes(_) => quote!(Bytes),
            Self::BigInt(_) => quote!(BigInt),
            Self::Decimal(_) => quote!(Decimal),
            Self::DateTime(_) => quote!(DateTime),
            Self::Timestamp(_) => quote!(Timestamp),
        };

        Ok(quote! {
            ::linq_rs::IrType::#name
        })
    }
}
