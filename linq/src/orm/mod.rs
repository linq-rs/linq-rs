//! ORM support types and helper methods.
//!
//! All `ORM` object must implement [`Table`] trait,
//! the proc_marco [`table`](linq_proc_macro::table)
//! will automate generate trait implementation for structures.
//!
//! Of course, you can also implement [`Table`] manually.
//! The framework does not prohibit users from doing so
//!

mod table;
pub use table::*;

mod dsl;
pub use dsl::*;

mod col_like;
pub use col_like::*;
