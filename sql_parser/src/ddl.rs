use quote::quote;
use syn::{parse::Parse, Token};

use crate::gen::CodeGen;

mod alter;
mod cols;
mod constraint;
mod create;
mod drop;
mod kw;
mod truncate;

pub enum DDL {
    Create(create::Create),
    Alter(alter::Alter),
    Drop(drop::Drop),
    Truncate(truncate::Truncate),
}

impl Parse for DDL {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        let ddl = if lookahead.peek(kw::CREATE) {
            DDL::Create(input.parse()?)
        } else if lookahead.peek(kw::ALTER) {
            DDL::Alter(input.parse()?)
        } else if lookahead.peek(kw::DROP) {
            DDL::Drop(input.parse()?)
        } else if lookahead.peek(kw::TRUNCATE) {
            DDL::Truncate(input.parse()?)
        } else {
            return Err(syn::Error::new(
                input.span(),
                "Expect CREATE/ALTER/DROP/TRUNCATE ",
            ));
        };

        if input.lookahead1().peek(Token!(;)) {
            let _: Token!(;) = input.parse()?;
        }

        Ok(ddl)
    }
}

impl CodeGen for DDL {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        match self {
            Self::Create(create) => return create.gen_ir_code(),
            Self::Alter(alter) => return alter.gen_ir_code(),
            Self::Drop(drop) => return drop.gen_ir_code(),
            Self::Truncate(truncate) => return truncate.gen_ir_code(),
        }
    }
}

pub struct DDLs {
    ddls: Vec<DDL>,
}

impl Parse for DDLs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut ddls = vec![];

        while !input.is_empty() {
            ddls.push(input.parse()?);
        }

        Ok(DDLs { ddls })
    }
}

impl CodeGen for DDLs {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let mut ddls = vec![];

        for ddl in &self.ddls {
            ddls.push(ddl.gen_ir_code()?);
        }

        Ok(quote! {
            vec![#(#ddls,)*]
        })
    }
}
