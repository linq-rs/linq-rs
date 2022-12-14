use syn::parse::Parse;

#[derive(Debug, Clone, PartialEq)]
pub struct Order {}

impl Parse for Order {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        unimplemented!()
    }
}
