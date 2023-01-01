mod select;
use quote::quote;
pub use select::*;

mod kw;
pub use kw::*;

mod cond;
pub use cond::*;

mod limit;
pub use limit::*;

mod order;
pub use order::*;

mod from;
pub use from::*;

mod insert;
pub use insert::*;

mod update;
pub use update::*;

mod cols;
pub use cols::*;

mod delete;
pub use delete::*;

use crate::CodeGen;

use linq_sql_parser::{RQLs, RQL};

impl CodeGen for RQL {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        match self {
            Self::Select(select) => return select.gen_ir_code(),
            Self::Insert(insert) => return insert.gen_ir_code(),
            Self::Update(update) => return update.gen_ir_code(),
            Self::Delete(delete) => return delete.gen_ir_code(),
        }
    }
}

impl CodeGen for RQLs {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let mut rqls = vec![];

        for rql in &self.rqls {
            rqls.push(rql.gen_ir_code()?);
        }

        Ok(quote! {
            (#(#rqls,)*)
        })
    }
}
