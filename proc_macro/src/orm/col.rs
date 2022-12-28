use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{parse::Parse, Attribute, LitStr, Token, Type};

use crate::gen::CodeGen;

mod kw {
    use syn::custom_keyword;

    custom_keyword!(from);
    custom_keyword!(to);
    custom_keyword!(autoinc);
}

#[derive(Clone)]
pub struct ColumnDef {
    pub name: Ident,
    pub attrs: Vec<ColumnAttr>,
    pub col_type: Type,
}

impl ColumnDef {
    pub fn col_name(&self) -> LitStr {
        for attr in &self.attrs {
            match attr {
                ColumnAttr::Name(name) => return name.clone(),
                _ => {}
            }
        }

        return LitStr::new(&self.name.to_string(), self.name.span());
    }
    pub fn col_type(&self) -> ColumnType {
        for attr in &self.attrs {
            match attr {
                ColumnAttr::OneToMany(_) => return ColumnType::OneToMany,
                ColumnAttr::OneToOne(_) => return ColumnType::OneToOne,
                ColumnAttr::Primary(_) => return ColumnType::Primary,
                _ => {}
            }
        }

        return ColumnType::Simple;
    }

    pub fn related(&self) -> syn::Result<Related> {
        for attr in &self.attrs {
            match attr {
                ColumnAttr::OneToMany(related) => return Ok(related.clone()),
                ColumnAttr::OneToOne(related) => return Ok(related.clone()),
                _ => {}
            }
        }

        return Err(syn::Error::new(
            self.name.span(),
            "Can't get related from simple column",
        ));
    }
}

impl CodeGen for ColumnDef {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let name = &self.name;
        let ty = &self.col_type;

        match self.col_type() {
            ColumnType::Primary => Ok(quote! {
                #name : ::linq_rs::orm::codegen::Column<#ty>
            }),
            ColumnType::Simple => Ok(quote! {
                #name : ::linq_rs::orm::codegen::Column<#ty>
            }),
            ColumnType::OneToOne => Ok(quote! {
                #name : ::linq_rs::orm::codegen::OneToOne<#ty>
            }),
            ColumnType::OneToMany => Ok(quote! {
                #name : ::linq_rs::orm::codegen::OneToMany<#ty>
            }),
        }
    }
}

pub enum ColumnType {
    Simple,
    OneToOne,
    OneToMany,
    Primary,
}

#[derive(Clone)]
pub enum ColumnAttr {
    Name(LitStr),
    OneToOne(Related),
    OneToMany(Related),
    Primary(bool),
}

impl ColumnAttr {
    pub fn new(field: &Ident, attr: &Attribute) -> syn::Result<Self> {
        if let Some(path) = attr.path.get_ident() {
            let name = path.to_string();
            match name.as_str() {
                "primary" => Self::parse_primary(field, attr),
                "column" => Self::parse_col_name(field, attr),
                "one_to_one" => Self::parse_one_to_one(field, attr),
                "one_to_many" => Self::parse_one_to_many(field, attr),
                _ => {
                    return Err(syn::Error::new(
                        path.span(),
                        format!("Unknown attr({})", path),
                    ));
                }
            }
        } else {
            return Err(syn::Error::new_spanned(
                attr,
                format!("Unknown attr({:?})", attr.to_token_stream().to_string()),
            ));
        }
    }

    fn parse_primary(_: &Ident, attr: &Attribute) -> syn::Result<Self> {
        if attr.tokens.is_empty() {
            return Ok(Self::Primary(false));
        }

        let auto_inc: Option<kw::autoinc> = attr.parse_args()?;

        Ok(Self::Primary(auto_inc.map(|_| true).unwrap_or(false)))
    }

    fn parse_col_name(field: &Ident, attr: &Attribute) -> syn::Result<Self> {
        Ok(Self::Name(attr.parse_args().map_err(|err| {
            syn::Error::new(
                err.span(),
                format!("parse col name attr({}) error: {}", field, err),
            )
        })?))
    }

    fn parse_one_to_one(field: &Ident, attr: &Attribute) -> syn::Result<Self> {
        Ok(Self::OneToOne(attr.parse_args().map_err(|err| {
            syn::Error::new(
                err.span(),
                format!("parse one_to_one({}) attr error: {}", field, err),
            )
        })?))
    }

    fn parse_one_to_many(field: &Ident, attr: &Attribute) -> syn::Result<Self> {
        Ok(Self::OneToMany(attr.parse_args().map_err(|err| {
            syn::Error::new(
                err.span(),
                format!("parse one_to_many({}) attr error: {}", field, err),
            )
        })?))
    }
}

#[derive(Clone)]
pub struct Related {
    pub from: Ident,
    pub to: Ident,
}

impl Parse for Related {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut from_col: Option<Ident> = None;
        let mut to_col: Option<Ident> = None;

        while !input.is_empty() {
            if input.lookahead1().peek(kw::from) {
                if from_col.is_some() {
                    return Err(syn::Error::new(input.span(), "Define from twice"));
                }

                input.parse::<kw::from>()?;
                input.parse::<Token!(=)>()?;
                from_col = Some(input.parse()?);

                continue;
            } else if input.lookahead1().peek(kw::to) {
                if to_col.is_some() {
                    return Err(syn::Error::new(input.span(), "Define to twice"));
                }

                input.parse::<kw::to>()?;
                input.parse::<Token!(=)>()?;
                to_col = Some(input.parse()?);

                continue;
            }

            return Err(syn::Error::new(
                input.span(),
                format!("Unexpect attr args: {}", input),
            ));
        }

        let from_col = from_col.ok_or(syn::Error::new(
            input.span(),
            "one_to_many/one_to_one expect args (from=xxx, to=xxx)",
        ))?;
        let to_col = to_col.ok_or(syn::Error::new(input.span(), "Expect to col"))?;

        Ok(Related {
            from: from_col,
            to: to_col,
        })
    }
}
