use proc_macro::TokenStream;
use syn::parse_macro_input;

mod dml;
use dml::*;

mod gen;
use gen::*;

mod ddl;
use ddl::*;

mod variant;

#[proc_macro]
pub fn rql(ident: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(ident as RQL);

    let token_stream = ast.gen_ir_code().expect("gen ir code");

    // eprintln!("gen: {}", token_stream.to_string());

    token_stream.into()
}

#[proc_macro]
pub fn rqls(ident: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(ident as RQLs);

    let token_stream = ast.gen_ir_code().expect("gen ir code");

    token_stream.into()
}

#[proc_macro]
#[allow(non_snake_case)]
pub fn rql_where(ident: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(ident as CondExpr);

    let token_stream = ast.gen_ir_code().expect("gen ir code");

    token_stream.into()
}

#[proc_macro]
pub fn ddl(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DDLs);

    let token_stream = ast.gen_ir_code().expect("Gen ir code error");

    // eprintln!("gen: {}", token_stream.to_string());

    token_stream.into()
}
