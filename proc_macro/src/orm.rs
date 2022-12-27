use syn::{parse::Parse, ItemStruct, LitStr};

use crate::gen::CodeGen;

mod structdef;
use structdef::*;

#[derive(Default)]
pub struct Table {
    table_name: Option<LitStr>,
    struct_def: Option<StructDef>,
}

impl Parse for Table {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            table_name: input.parse()?,
            ..Default::default()
        })
    }
}

impl Table {
    pub fn with_struct_define(&mut self, item: ItemStruct) -> syn::Result<&mut Self> {
        self.struct_def = Some(StructDef::new(&item)?);

        Ok(self)
    }
}

impl CodeGen for Table {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        self.struct_def.as_ref().unwrap().gen_ir_code()
    }
}
