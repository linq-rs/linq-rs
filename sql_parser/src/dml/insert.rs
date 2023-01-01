use syn::parse::Parse;

use super::{kw, Columns, Variant};

pub struct Insert {
    pub table_name: Variant,
    pub cols: Columns,
}

impl Parse for Insert {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: kw::INSERT = input.parse()?;

        let _: kw::INTO = input.parse()?;

        let table_name: Variant = input.parse()?;

        let cols = input.parse()?;

        Ok(Insert { table_name, cols })
    }
}
