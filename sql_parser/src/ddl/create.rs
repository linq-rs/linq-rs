use std::collections::HashSet;

use quote::quote;
use syn::{parenthesized, parse::Parse, Token};

use crate::gen::CodeGen;

use super::{cols::Column, constraint::NamedConstraint, kw};

use crate::variant::Variant;

pub struct Create {
    table_name: Variant,
    cols: Vec<Column>,
    constraints: Vec<NamedConstraint>,
}

impl CodeGen for Create {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let table_name = self.table_name.gen_ir_code()?;

        let mut cols = vec![];

        for col in &self.cols {
            cols.push(col.gen_ir_code()?);
        }

        let mut constraints = vec![];

        for c in &self.constraints {
            constraints.push(c.gen_ir_code()?);
        }

        Ok(quote! {
            ::linq_rs::ddl::DDL::Create(::linq_rs::ddl::Create {
                table_name: #table_name,
                cols: vec![#(#cols,)*],
                constraints: vec![#(#constraints,)*],
            })
        })
    }
}

impl Parse for Create {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: kw::CREATE = input.parse()?;

        let _: kw::TABLE = input.parse()?;

        let table_name = input.parse()?;

        let content;

        parenthesized!(content in input);

        let mut cols = vec![];
        let mut col_names = HashSet::new();

        while !content.is_empty() {
            let col: Column = content.parse()?;

            if col_names.contains(&col.name) {
                return Err(syn::Error::new(col.span, "Define col twice"));
            }

            col_names.insert(col.name.to_owned());

            cols.push(col);

            if !content.lookahead1().peek(Token![,]) {
                break;
            }

            let _: Token!(,) = content.parse()?;

            if content.lookahead1().peek(kw::CONSTRAINT) {
                break;
            }
        }

        let mut constraints: Vec<NamedConstraint> = vec![];
        let mut constraint_names = HashSet::<String>::new();

        while !content.is_empty() && content.lookahead1().peek(kw::CONSTRAINT) {
            let constraint: NamedConstraint = content.parse()?;

            if constraint_names.contains(&constraint.name) {
                return Err(syn::Error::new(constraint.span, "Define constraint twice"));
            }

            constraint_names.insert(constraint.name.clone());

            constraints.push(constraint);

            if !content.lookahead1().peek(Token![,]) {
                break;
            }

            let _: Token!(,) = content.parse()?;
        }

        for constraint in &constraints {
            constraint.verify(&col_names)?;
        }

        Ok(Create {
            table_name,
            cols,
            constraints,
        })
    }
}
