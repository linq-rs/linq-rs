use proc_macro2::Ident;

use syn::parse::Parse;

use super::{cols::Column, constraint::NamedConstraint, kw};

pub struct Alter {
    pub table_name: String,
    pub expr: AlterExpr,
}

impl Parse for Alter {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: kw::ALTER = input.parse()?;
        let _: kw::TABLE = input.parse()?;

        let table_name: Ident = input.parse()?;

        Ok(Alter {
            table_name: table_name.to_string(),
            expr: input.parse()?,
        })
    }
}

pub enum AlterExpr {
    AddColumn(Column),
    DropColumn(String),
    AlterColumn(Column),
    AddConstraint(NamedConstraint),
    AlterConstraint(NamedConstraint),
    DropConstraint(String),
    RenameTable(String),
    RenameColumn(String, String),
    RenameConstraint(String, String),
}

impl Parse for AlterExpr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::ADD) {
            let _: kw::ADD = input.parse()?;

            if input.lookahead1().peek(kw::CONSTRAINT) {
                return Ok(Self::AddConstraint(input.parse()?));
            } else {
                let _: kw::COLUMN = input.parse()?;
                return Ok(Self::AddColumn(input.parse()?));
            }
        } else if lookahead.peek(kw::DROP) {
            let _: kw::DROP = input.parse()?;

            if input.lookahead1().peek(kw::CONSTRAINT) {
                let _: kw::CONSTRAINT = input.parse()?;

                let column_name: Ident = input.parse()?;

                return Ok(Self::DropConstraint(column_name.to_string()));
            } else {
                let _: kw::COLUMN = input.parse()?;

                let column_name: Ident = input.parse()?;

                return Ok(Self::DropColumn(column_name.to_string()));
            }
        } else if lookahead.peek(kw::ALTER) {
            let _: kw::ALTER = input.parse()?;

            if input.lookahead1().peek(kw::CONSTRAINT) {
                return Ok(Self::AlterConstraint(input.parse()?));
            } else {
                let _: kw::COLUMN = input.parse()?;
                return Ok(Self::AlterColumn(input.parse()?));
            }
        } else if lookahead.peek(kw::RENAME) {
            let _: kw::RENAME = input.parse()?;

            let lookahead = input.lookahead1();

            if lookahead.peek(kw::CONSTRAINT) {
                let _: kw::CONSTRAINT = input.parse()?;

                let from: Ident = input.parse()?;

                let _: kw::TO = input.parse()?;

                let to: Ident = input.parse()?;

                return Ok(Self::RenameConstraint(from.to_string(), to.to_string()));
            } else if lookahead.peek(kw::COLUMN) {
                let _: kw::COLUMN = input.parse()?;

                let from: Ident = input.parse()?;

                let _: kw::TO = input.parse()?;

                let to: Ident = input.parse()?;

                return Ok(Self::RenameColumn(from.to_string(), to.to_string()));
            } else {
                let _: kw::TABLE = input.parse()?;

                let _: kw::TO = input.parse()?;

                let to: Ident = input.parse()?;

                return Ok(Self::RenameTable(to.to_string()));
            }
        }

        unimplemented!()
    }
}
