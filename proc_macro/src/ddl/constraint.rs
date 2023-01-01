use quote::quote;

use crate::gen::CodeGen;

use linq_sql_parser::{ColumnNames, Constraint, NamedConstraint};

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

impl CodeGen for ColumnNames {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let names = &self.names;

        Ok(quote! {
            vec![#(#names,)*]
        })
    }
}
