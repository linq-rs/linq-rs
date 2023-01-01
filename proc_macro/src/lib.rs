use proc_macro::TokenStream;
use syn::{parse_macro_input, LitStr};

mod dml;
use dml::*;

mod gen;
use gen::*;

mod ddl;
use ddl::*;

mod variant;

mod orm;
use orm::*;

mod utils;

/// Building Single-Row SQL dml statements
#[proc_macro]
pub fn rql(ident: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(ident as RQL);

    let token_stream = ast.gen_ir_code().expect("gen ir code");

    // eprintln!("gen: {}", token_stream.to_string());

    token_stream.into()
}

/// Building multi-line SQL dml statements
#[proc_macro]
pub fn rqls(ident: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(ident as RQLs);

    let token_stream = ast.gen_ir_code().expect("gen ir code");

    token_stream.into()
}

/// Building SQL dml [`where`](https://www.w3schools.com/sql/sql_where.asp) clause
#[proc_macro]
#[allow(non_snake_case)]
pub fn rql_where(ident: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(ident as CondExpr);

    let token_stream = ast.gen_ir_code().expect("gen ir code");

    token_stream.into()
}

/// Building multi-line SQL DDL statements
#[proc_macro]
pub fn ddl(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DDLs);

    let token_stream = ast.gen_ir_code().expect("Gen ir code error");

    // eprintln!("gen: {}", token_stream.to_string());

    token_stream.into()
}

#[proc_macro_attribute]
pub fn table(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let table_name = parse_macro_input!(attrs as Option<LitStr>);

    let token_stream = Table::new(table_name, parse_macro_input!(item))
        .expect("Parse table struct error")
        .gen_ir_code()
        .expect("Generate table code error");

    // eprintln!("{}", token_stream);

    token_stream.into()
}
