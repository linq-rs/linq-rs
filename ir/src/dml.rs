mod col;
pub use col::*;

mod cond;
pub use cond::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Selecter<'a> {
    pub cols: SelectColumns<'a>,
}
