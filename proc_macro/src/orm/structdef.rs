use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{parse::Parse, Attribute, Fields, ItemStruct, LitStr, Token, Type, Visibility};

use crate::gen::CodeGen;

pub struct StructDef {
    ident: Ident,
    cols: Vec<ColumnDef>,
    primary_field: Ident,
    auto_inc: bool,
    vis: Visibility,
}

impl StructDef {
    pub fn new(item: &ItemStruct) -> syn::Result<Self> {
        let mut cols = vec![];

        let mut primary_col = None;

        if let Fields::Named(fields) = &item.fields {
            for field in &fields.named {
                let mut attrs = vec![];

                for attr in &field.attrs {
                    let col_attr = ColumnAttr::new(attr)?;

                    if let ColumnAttr::Primary(auto_inc) = col_attr {
                        if primary_col.is_some() {
                            return Err(syn::Error::new_spanned(attr, "Duplicate primary defined"));
                        }

                        primary_col = Some((field.ident.clone().unwrap(), auto_inc));
                    }

                    attrs.push(col_attr);
                }

                cols.push(ColumnDef {
                    name: field.ident.clone().unwrap(),
                    attrs,
                    col_type: field.ty.clone(),
                })
            }
        }

        let (primary_field, auto_inc) = primary_col.ok_or(syn::Error::new(
            item.ident.span(),
            "Table must define primary col",
        ))?;

        for col in &cols {
            if col.name == primary_field {
                for attr in &col.attrs {
                    match attr {
                        ColumnAttr::OneToMany(_) => {
                            return Err(syn::Error::new(
                                primary_field.span(),
                                "Tag primary key on one_to_many col",
                            ));
                        }
                        ColumnAttr::OneToOne(_) => {
                            return Err(syn::Error::new(
                                primary_field.span(),
                                "Tag primary key on one_to_one col",
                            ));
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(StructDef {
            ident: item.ident.clone(),
            cols,
            primary_field,
            auto_inc,
            vis: item.vis.clone(),
        })
    }
}

impl CodeGen for StructDef {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let vis = &self.vis;
        let ident = &self.ident;

        let mut cols = vec![];

        for col in &self.cols {
            cols.push(col.gen_ir_code()?);
        }

        Ok(quote! {
            #vis struct #ident {
                #(#cols,)*
            }
        })
    }
}

mod kw {
    use syn::custom_keyword;

    custom_keyword!(from);
    custom_keyword!(to);
    custom_keyword!(autoinc);
}

#[derive(Clone)]
struct ColumnDef {
    name: Ident,
    attrs: Vec<ColumnAttr>,
    col_type: Type,
}

impl CodeGen for ColumnDef {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        unimplemented!()
    }
}

#[derive(Clone)]
enum ColumnAttr {
    Name(LitStr),
    OneToOne(Related),
    OneToMany(Related),
    Primary(bool),
}

impl ColumnAttr {
    pub fn new(attr: &Attribute) -> syn::Result<Self> {
        if let Some(path) = attr.path.get_ident() {
            let name = path.to_string();
            match name.as_str() {
                "primary" => Self::parse_primary(attr),
                "column" => Self::parse_col_name(attr),
                "one_to_one" => Self::parse_one_to_one(attr),
                "one_to_many" => Self::parse_one_to_many(attr),
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

    fn parse_primary(attr: &Attribute) -> syn::Result<Self> {
        if attr.tokens.is_empty() {
            return Ok(Self::Primary(false));
        }

        let auto_inc: Option<kw::autoinc> = attr.parse_args()?;

        Ok(Self::Primary(auto_inc.map(|_| true).unwrap_or(false)))
    }

    fn parse_col_name(attr: &Attribute) -> syn::Result<Self> {
        Ok(Self::Name(attr.parse_args()?))
    }

    fn parse_one_to_one(attr: &Attribute) -> syn::Result<Self> {
        Ok(Self::OneToOne(attr.parse_args()?))
    }

    fn parse_one_to_many(attr: &Attribute) -> syn::Result<Self> {
        Ok(Self::OneToMany(attr.parse_args()?))
    }
}

#[derive(Clone)]
struct Related {
    from: Ident,
    to: Ident,
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

                input.parse::<kw::from>()?;
                input.parse::<Token!(=)>()?;
                to_col = Some(input.parse()?);

                continue;
            }

            return Err(syn::Error::new(input.span(), "Expect from/to attr value"));
        }

        let from_col = from_col.ok_or(syn::Error::new(input.span(), "Expect from col"))?;
        let to_col = to_col.ok_or(syn::Error::new(input.span(), "Expect to col"))?;

        Ok(Related {
            from: from_col,
            to: to_col,
        })
    }
}
