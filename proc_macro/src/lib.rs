use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn rql(ident: TokenStream) -> TokenStream {
    eprintln!("sql string {}", ident.to_string());
    quote!().into()
}
