use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Fields, ItemStruct, LitStr, Visibility};

use crate::gen::CodeGen;

mod col;
use col::*;

#[allow(dead_code)]
pub struct Table {
    table_name: Option<LitStr>,
    ident: Ident,
    cols: Vec<ColumnDef>,
    primary_field: Ident,
    auto_inc: bool,
    vis: Visibility,
}

impl Table {
    pub fn new(table_name: Option<LitStr>, item: ItemStruct) -> syn::Result<Self> {
        let mut cols = vec![];

        let mut primary_col = None;

        if let Fields::Named(fields) = &item.fields {
            for field in &fields.named {
                let mut attrs = vec![];

                for attr in &field.attrs {
                    let col_attr = ColumnAttr::new(&field.ident.clone().unwrap(), attr)?;

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

        Ok(Self {
            table_name,
            ident: item.ident.clone(),
            cols,
            primary_field,
            auto_inc,
            vis: item.vis.clone(),
        })
    }
}

impl Table {
    fn gen_struct_define(&self) -> syn::Result<TokenStream> {
        let vis = &self.vis;
        let ident = &self.ident;

        let mut cols = vec![];

        for col in &self.cols {
            cols.push(col.gen_ir_code()?);
        }

        Ok(quote! {
            #[derive(Default)]
            #vis struct #ident {
                #(pub #cols,)*
            }
        })
    }

    fn gen_impl_table(&self) -> syn::Result<TokenStream> {
        let ident = &self.ident;

        let table_name = self.gen_table_name_fn()?;

        let table_name_const = self.gen_table_name_const_fn()?;

        let cols = self.gen_cols_fn()?;

        let write = self.gen_write_fn()?;

        let read = self.gen_read_fn()?;

        Ok(quote! {
            impl ::linq_rs::orm::Table for #ident {
                #table_name

                #cols

                #write

                #read
            }

            impl #ident {
                #table_name_const
            }
        })
    }

    fn gen_cols_fn(&self) -> syn::Result<TokenStream> {
        let mut cols = vec![];

        for col in &self.cols {
            let col_name = col.col_name();

            cols.push(match col.col_type() {
                ColumnType::Simple => {
                    quote!(::linq_rs::orm::Column::Simple(#col_name))
                }
                ColumnType::Primary => {
                    let auto_inc = self.auto_inc;
                    quote!(::linq_rs::orm::Column::Primary(#col_name,#auto_inc))
                }
                ColumnType::OneToOne => {
                    let related = col.related()?;
                    let ref_col = related.from.to_string();
                    let col_type = &col.col_type;
                    let foreign_key_col = related.to.to_string();

                    quote!(::linq_rs::orm::Column::OneToOne(::linq_rs::orm::Cascade {
                       name: #col_name,
                       ref_col: #ref_col,
                       table_name: #col_type::table_name_const(),
                       foreign_key_col: #foreign_key_col
                    }))
                }
                ColumnType::OneToMany => {
                    let related = col.related()?;
                    let ref_col = related.from.to_string();
                    let col_type = &col.col_type;
                    let foreign_key_col = related.to.to_string();

                    quote!(::linq_rs::orm::Column::OneToMany(::linq_rs::orm::Cascade {
                       name: #col_name,
                       ref_col: #ref_col,
                       table_name: #col_type::table_name_const(),
                       foreign_key_col: #foreign_key_col
                    }))
                }
            });
        }

        Ok(quote! {
            fn cols() -> &'static [::linq_rs::orm::Column] {
                static COLS: &'static [::linq_rs::orm::Column] = &[#(#cols,)*];

                COLS
            }
        })
    }

    fn gen_table_name_fn(&self) -> syn::Result<TokenStream> {
        let mut table_name = LitStr::new(&self.ident.to_string(), self.ident.span());

        if let Some(name) = self.table_name.as_ref() {
            table_name = name.clone()
        }

        Ok(quote! {
            fn table_name() -> &'static str {
                #table_name
            }
        })
    }

    fn gen_table_name_const_fn(&self) -> syn::Result<TokenStream> {
        let mut table_name = LitStr::new(&self.ident.to_string(), self.ident.span());

        if let Some(name) = self.table_name.as_ref() {
            table_name = name.clone()
        }

        Ok(quote! {
            const fn table_name_const() -> &'static str {
                #table_name
            }
        })
    }

    fn gen_write_fn(&self) -> syn::Result<TokenStream> {
        let mut cols = vec![];

        for col in &self.cols {
            let col_name = col.col_name();
            let ident = &col.name;

            cols.push(quote! {
                #col_name => {
                    self.#ident.from_column_value(value)?;
                }
            });
        }

        Ok(quote! {
            fn from_values(&mut self, values: Vec<::linq_rs::orm::ColumnValue>) -> ::linq_rs::anyhow::Result<()> {
                use ::linq_rs::orm::codegen::ColumnLike;

                for value in values {
                    match value.col_name() {
                        #(#cols,)*
                        _ => {
                            return Err(::linq_rs::anyhow::format_err!("Unknown col: {}",value.col_name()));
                        }
                    }
                }

                Ok(())
            }
        })
    }

    fn gen_read_fn(&self) -> syn::Result<TokenStream> {
        let mut cols = vec![];

        for col in &self.cols {
            let col_name = col.col_name();
            let ident = &col.name;

            cols.push(quote! {
                self.#ident.into_column_value(#col_name)
            });
        }

        Ok(quote! {
            fn into_values(&mut self) ->  Vec<::linq_rs::orm::ColumnValue> {
                use ::linq_rs::orm::codegen::ColumnLike;

                vec![#(#cols,)*]
            }
        })
    }
}

impl CodeGen for Table {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        let struct_define = self.gen_struct_define()?;
        let impl_table = self.gen_impl_table()?;

        Ok(quote! {
            #struct_define

            #impl_table
        })
    }
}
