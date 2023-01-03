use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{Fields, ItemStruct, LitStr, Visibility};

use crate::gen::CodeGen;
use linq_sql_parser::extract_type_from_vec;

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
    item: ItemStruct,
}

impl Table {
    pub fn new(item: ItemStruct) -> syn::Result<Self> {
        let mut table_name = None;

        for attr in &item.attrs {
            if let Some(path) = attr.path.get_ident() {
                let name = path.to_string();
                match name.as_str() {
                    "table_name" => {
                        table_name = Some(attr.parse_args()?);
                    }
                    _ => {}
                };
            }
        }

        let mut cols = vec![];

        let mut primary_col = None;

        if let Fields::Named(fields) = &item.fields {
            for field in &fields.named {
                let mut attrs = vec![];

                for attr in &field.attrs {
                    if let Some(col_attr) = ColumnAttr::new(&field.ident.clone().unwrap(), attr)? {
                        if let ColumnAttr::Primary(auto_inc) = col_attr {
                            if primary_col.is_some() {
                                return Err(syn::Error::new_spanned(
                                    attr,
                                    "Duplicate primary defined",
                                ));
                            }

                            primary_col = Some((field.ident.clone().unwrap(), auto_inc));
                        }

                        attrs.push(col_attr);
                    }
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
                        ColumnAttr::Cascade(_) => {
                            return Err(syn::Error::new(
                                primary_field.span(),
                                "Tag primary key on one_to_many col",
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
            item,
        })
    }
}

impl Table {
    #[allow(unused)]
    fn gen_struct_define(&self) -> syn::Result<TokenStream> {
        let vis = &self.vis;
        let ident = &self.ident;

        let mut cols = vec![];

        for col in &self.cols {
            cols.push(col.gen_ir_code()?);
        }

        let attrs = &self.item.attrs;

        Ok(quote! {
            #(#attrs)*
            #[derive(Default)]
            #vis struct #ident {
                #(pub #cols,)*
            }
        })
    }

    fn gen_impl_table(&self) -> syn::Result<TokenStream> {
        let ident = &self.ident;

        let table_name = self.gen_table_name_fn()?;

        let cols = self.gen_cols_fn()?;

        let write = self.gen_write_fn()?;

        let read = self.gen_read_fn()?;

        let col_names = self.gen_col_name_fns()?;

        Ok(quote! {
            impl ::linq_rs::orm::Table for #ident {
                #table_name

                #cols

                #write

                #read
            }

            impl #ident {
                #col_names
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
                ColumnType::Cascade => {
                    let related = col.related()?;
                    let ref_col_name_fn =
                        format_ident!("{}", related.from, span = related.from.span());
                    let col_type = &col.col_type;

                    let foreign_key_col_name_fn =
                        format_ident!("{}", related.to, span = related.to.span());

                    let self_type = &self.ident;

                    if let Some(vec_type) = extract_type_from_vec(col_type) {
                        quote!(::linq_rs::orm::Column::OneToOne(::linq_rs::orm::Cascade {
                           name: #col_name,
                           ref_col: #self_type::#ref_col_name_fn(),
                           table_name: || #vec_type::table_name(),
                           foreign_key_col: #vec_type::#foreign_key_col_name_fn(),
                           table_cols: || #vec_type::cols(),
                        }))
                    } else {
                        quote!(::linq_rs::orm::Column::OneToOne(::linq_rs::orm::Cascade {
                           name: #col_name,
                           ref_col: #self_type::#ref_col_name_fn(),
                           table_name: || #col_type::table_name(),
                           foreign_key_col: #col_type::#foreign_key_col_name_fn(),
                           table_cols: || #col_type::cols(),
                        }))
                    }
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

    fn gen_write_fn(&self) -> syn::Result<TokenStream> {
        let mut cols = vec![];

        let mut idents = vec![];

        for col in &self.cols {
            let col_name = col.col_name();
            let ident = &col.name;
            let ty = &col.col_type;

            cols.push(quote! {
                assert_eq!(values[0].col_name(), #col_name);
                let #ident = ::linq_rs::orm::from_column_value::<#ty>(values.remove(0))?;
            });

            idents.push(ident);
        }

        let count = self.cols.len();

        Ok(quote! {
            fn from_values(mut values: Vec<::linq_rs::orm::ColumnValue>) -> ::linq_rs::anyhow::Result<Self> {
                use ::linq_rs::orm::ColumnLike;

                assert_eq!(values.len(), #count);

                #(#cols)*

                Ok(Self {
                    #(#idents,)*
                })
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
            fn into_values(mut self) ->  Vec<::linq_rs::orm::ColumnValue> {
                use ::linq_rs::orm::ColumnLike;

                vec![#(#cols,)*]
            }
        })
    }

    fn gen_col_name_fns(&self) -> syn::Result<TokenStream> {
        let mut cols = vec![];

        for col in &self.cols {
            let col_name = col.col_name();
            let ident = &col.name;

            let fn_name = format_ident!("col_{}", ident, span = ident.span());

            cols.push(quote! {
                const fn #fn_name() -> &'static str {
                    #col_name
                }
            });
        }

        Ok(quote! {
            #(#cols)*
        })
    }
}

impl CodeGen for Table {
    fn gen_ir_code(&self) -> syn::Result<proc_macro2::TokenStream> {
        // let struct_define = self.gen_struct_define()?;
        let impl_table = self.gen_impl_table()?;

        Ok(quote! {
            // #struct_define

            #impl_table
        })
    }
}
