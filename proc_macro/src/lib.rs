use proc_macro::TokenStream;
use syn::parse_macro_input;

mod rql;
use rql::*;

mod gen;
use gen::*;

#[proc_macro]
pub fn rql(ident: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(ident as RQL);

    let token_stream = ast.gen_ir_code().expect("gen ir code");

    eprintln!("{}", token_stream.to_string());

    token_stream.into()
}
