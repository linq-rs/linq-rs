use std::collections::HashSet;

use proc_macro2::{Ident, Span};
use quote::quote;
use syn::parse::Parse;
use syn::{parenthesized, Token};

use crate::gen::CodeGen;
use crate::variant::Variant;

use super::kw;

pub struct NamedConstraint {
    pub span: Span,
    pub name: String,
    constraint: Constraint,
}

impl Parse for NamedConstraint {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let span = input.span();

        let _: kw::CONSTRAINT = input.parse()?;

        let name: Ident = input.parse()?;

        Ok(Self {
            span,
            name: name.to_string(),
            constraint: input.parse()?,
        })
    }
}

impl NamedConstraint {
    pub fn verify(&self, col_names: &HashSet<String>) -> syn::Result<()> {
        match &self.constraint {
            Constraint::Index(names) => names.verify(col_names),
            Constraint::Unique(names) => names.verify(col_names),
            Constraint::ForeignKey(names, _, _) => names.verify(col_names),
        }
    }
}

impl CodeGen for NamedConstraint {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let name = &self.name;
        match &self.constraint {
            Constraint::Index(cols) => {
                let cols = cols.gen_ir_code()?;
                Ok(quote! {
                    ::linq_rs::ddl::NamedConstraint {
                        name: #name,
                        constraint: ::linq_rs::ddl::Constraint::Index(#cols),
                    }
                })
            }
            Constraint::Unique(cols) => {
                let cols = cols.gen_ir_code()?;
                Ok(quote! {
                    ::linq_rs::ddl::NamedConstraint {
                        name: #name,
                        constraint: ::linq_rs::ddl::Constraint::Unique(#cols),
                    }
                })
            }
            Constraint::ForeignKey(cols, ref_table, ref_cols) => {
                let cols = cols.gen_ir_code()?;
                let ref_table = ref_table.gen_ir_code()?;
                let ref_cols = ref_cols.gen_ir_code()?;

                Ok(quote! {
                    ::linq_rs::ddl::NamedConstraint {
                        name: #name,
                        constraint: ::linq_rs::ddl::Constraint::ForeignKey(#cols,#ref_table,#ref_cols),
                    }
                })
            }
        }
    }
}

enum Constraint {
    Unique(ColumnNames),
    Index(ColumnNames),
    ForeignKey(ColumnNames, Variant, ColumnNames),
}

impl Parse for Constraint {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::UNIQUE) {
            let _: kw::UNIQUE = input.parse()?;
            return Ok(Self::Unique(input.parse()?));
        } else if lookahead.peek(kw::INDEX) {
            let _: kw::INDEX = input.parse()?;
            return Ok(Self::Index(input.parse()?));
        } else if lookahead.peek(kw::FOREIGN) {
            let _: kw::FOREIGN = input.parse()?;
            let _: kw::KEY = input.parse()?;

            let cols = input.parse()?;

            let _: kw::REFERENCES = input.parse()?;

            let ref_table = input.parse()?;

            let ref_cols = input.parse()?;

            return Ok(Self::ForeignKey(cols, ref_table, ref_cols));
        }

        return Err(syn::Error::new(
            input.span(),
            "Expect keyword UNIQUE/INDEX/FOREIGN KEY",
        ));
    }
}

struct ColumnNames {
    spans: Vec<Span>,
    names: Vec<String>,
}

impl ColumnNames {
    fn verify(&self, col_names: &HashSet<String>) -> syn::Result<()> {
        for (index, name) in self.names.iter().enumerate() {
            if !col_names.contains(name) {
                return Err(syn::Error::new(self.spans[index], "Unknown col"));
            }
        }

        Ok(())
    }
}

impl Parse for ColumnNames {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);

        let mut cols = vec![];
        let mut spans = vec![];

        while !content.is_empty() {
            spans.push(content.span());
            let col: Ident = content.parse()?;

            cols.push(col.to_string());

            if content.lookahead1().peek(Token!(,)) {
                let _: Token!(,) = content.parse()?;
            } else {
                break;
            }
        }

        if !content.is_empty() {
            return Err(syn::Error::new(content.span(), "expect )"));
        }

        Ok(ColumnNames { names: cols, spans })
    }
}

impl CodeGen for ColumnNames {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let names = &self.names;

        Ok(quote! {
            vec![#(#names,)*]
        })
    }
}
