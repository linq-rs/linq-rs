use std::collections::HashSet;

use syn::{parenthesized, parse::Parse, Token};

use super::{cols::Column, constraint::NamedConstraint, kw};

use crate::variant::Variant;

pub struct Create {
    pub table_name: Variant,
    pub cols: Vec<Column>,
    pub constraints: Vec<NamedConstraint>,
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
