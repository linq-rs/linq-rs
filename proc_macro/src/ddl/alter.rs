use quote::quote;

use crate::gen::CodeGen;

use linq_sql_parser::{Alter, AlterExpr};

impl CodeGen for Alter {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let table_name = &self.table_name;
        let expr = self.expr.gen_ir_code()?;
        Ok(quote! {
            ::linq_rs::ddl::DDL::Alter(::linq_rs::ddl::Alter {
                table_name: #table_name,
                expr: #expr,
            })
        })
    }
}

impl CodeGen for AlterExpr {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        match self {
            Self::AddColumn(col) => {
                let col = col.gen_ir_code()?;

                Ok(quote! {
                    ::linq_rs::ddl::AlterExpr::AddColumn(#col)
                })
            }
            Self::DropColumn(col_name) => Ok(quote! {
                ::linq_rs::ddl::AlterExpr::DropColumn(#col_name)
            }),
            Self::AlterColumn(col) => {
                let col = col.gen_ir_code()?;

                Ok(quote! {
                    ::linq_rs::ddl::AlterExpr::AlterColumn(#col)
                })
            }
            Self::AddConstraint(constraint) => {
                let constraint = constraint.gen_ir_code()?;

                Ok(quote! {
                    ::linq_rs::ddl::AlterExpr::AddConstraint(#constraint)
                })
            }
            Self::AlterConstraint(constraint) => {
                let constraint = constraint.gen_ir_code()?;

                Ok(quote! {
                    ::linq_rs::ddl::AlterExpr::AlterConstraint(#constraint)
                })
            }
            Self::DropConstraint(constraint) => Ok(quote! {
                ::linq_rs::ddl::AlterExpr::DropConstraint(#constraint)
            }),
            Self::RenameConstraint(from, to) => Ok(quote! {
                ::linq_rs::ddl::AlterExpr::RenameConstraint(#from,#to)
            }),
            Self::RenameColumn(from, to) => Ok(quote! {
                ::linq_rs::ddl::AlterExpr::RenameConstraint(#from,#to)
            }),
            Self::RenameTable(to) => Ok(quote! {
                ::linq_rs::ddl::AlterExpr::RenameTable(#to)
            }),
        }
    }
}
