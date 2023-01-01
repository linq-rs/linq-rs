//! ORM support types and helper methods.
//!
//! All `ORM` object must implement [`Table`] trait,
//! the proc_marco [`table`](linq_proc_macro::table)
//! will automate generate trait implementation for structures.
//!
//! Of course, you can also implement [`Table`] manually.
//! The framework does not prohibit users from doing so
//!
//! # Examples
//!
//! ```
//! #[table]
//! // Support serde
//! #[derive(Serialize, Deserialize)]
//! struct User {
//!     #[column("id_")]
//!     #[primary(autoinc)]
//!     id: Option<i32>,
//!     first_name: String,
//!     last_name: String,
//!     #[cascade(from=col_id to=col_user_id)]
//!     cards: Vec<Card>,
//!     created_time: Option<DateTime>,
//!     updated_time: Option<DateTime>,
//! }
//!
//! ```

mod table;
pub use table::*;

mod dsl;
pub use dsl::*;

mod col_like;
pub use col_like::*;
