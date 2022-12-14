use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod rql;
use rql::*;

#[proc_macro]
pub fn rql(ident: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(ident as RQL);

    // eprintln!("{:?}", ast);

    quote!().into()
}
