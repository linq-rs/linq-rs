use proc_macro2::{Ident, Span};
use quote::quote;
use syn::token::Paren;
use syn::{parse::Parse, Token};

use crate::gen::CodeGen;
use crate::variant::Variant;

use super::kw;

pub struct Column {
    pub span: Span,
    pub name: String,
    col_type: IrType,
    not_null: bool,
    default_value: Option<Variant>,
    primary: Option<bool>,
}

impl Parse for Column {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let span = input.span();

        let name: Ident = input.parse()?;

        let name = name.to_string();

        let col_type = input.parse()?;

        let mut not_null = None;

        let mut default_value = None;

        let mut primary = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();

            if lookahead.peek(Token![;])
                || lookahead.peek(Token![,])
                || lookahead.peek(Paren)
                || lookahead.peek(kw::CONSTRAINT)
            {
                break;
            }

            if lookahead.peek(kw::NOT) {
                if not_null.is_some() {
                    return Err(syn::Error::new(input.span(), "NOT_NULL flag define twice"));
                }
                let _: kw::NOT = input.parse()?;
                let _: kw::NULL = input.parse()?;
                not_null = Some(true);

                continue;
            }

            if lookahead.peek(kw::PRIMARY) {
                if primary.is_some() {
                    return Err(syn::Error::new(input.span(), "PRIMARY flag define twice"));
                }
                let _: kw::PRIMARY = input.parse()?;

                if input.peek(kw::AUTOINC) {
                    let _: kw::AUTOINC = input.parse()?;
                    primary = Some(true);
                } else {
                    primary = Some(false);
                }

                continue;
            }

            if default_value.is_some() {
                return Err(syn::Error::new(input.span(), "Default value define twice"));
            }

            default_value = Some(input.parse()?);
        }

        Ok(Column {
            span,
            name,
            col_type,
            default_value,
            not_null: not_null.unwrap_or(false),
            primary,
        })
    }
}

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

enum IrType {
    Int(kw::INT),
    BigInt(kw::BIGINT),
    Float(kw::FLOAT),
    Decimal(kw::DECIMAL),
    String(kw::STRING),
    Bytes(kw::BYTES),
    DateTime(kw::DATETIME),
    Timestamp(kw::TIMESTAMP),
}

impl Parse for IrType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::INT) {
            Ok(Self::Int(input.parse()?))
        } else if lookahead.peek(kw::FLOAT) {
            Ok(Self::Float(input.parse()?))
        } else if lookahead.peek(kw::STRING) {
            Ok(Self::String(input.parse()?))
        } else if lookahead.peek(kw::BYTES) {
            Ok(Self::Bytes(input.parse()?))
        } else if lookahead.peek(kw::BIGINT) {
            Ok(Self::BigInt(input.parse()?))
        } else if lookahead.peek(kw::DECIMAL) {
            Ok(Self::Decimal(input.parse()?))
        } else if lookahead.peek(kw::DATETIME) {
            Ok(Self::DateTime(input.parse()?))
        } else if lookahead.peek(kw::TIMESTAMP) {
            Ok(Self::Timestamp(input.parse()?))
        } else {
            Err(syn::Error::new(input.span(), "Unknown column type"))
        }
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
