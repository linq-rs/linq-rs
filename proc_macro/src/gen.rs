use proc_macro2::TokenStream;

/// IR code generator
pub trait CodeGen {
    fn gen_ir_code(&self) -> syn::Result<TokenStream>;
}
