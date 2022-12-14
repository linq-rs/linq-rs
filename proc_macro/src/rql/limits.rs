use syn::parse::Parse;

#[derive(Debug, Clone, PartialEq)]
pub struct Limits {}

impl Parse for Limits {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        unimplemented!()
    }
}
