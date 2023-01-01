mod variant;
pub use variant::*;

/// This module define LINQ ir code executor traits.
pub mod driver;

/// This module define LINQ IR structures for SQL [`DML`](https://www.javatpoint.com/dbms-sql-command) clause
pub mod dml;

/// This module define LINQ IR structures for SQL [`DDL`](https://www.javatpoint.com/dbms-sql-command) clause
pub mod ddl;

/// LINQ to support orm api.
pub mod orm;

pub use anyhow;

pub use orm::{DeleteObject, DeleteWhereCond, Insert, Select, Update};

/// Proc macros to build LINQ language ir data.
pub use linq_proc_macro::ddl;

pub use linq_proc_macro::rql;

pub use linq_proc_macro::rqls;

pub use linq_proc_macro::rql_where;

/// Macro to generate implementation of the `Table` trait for data structures.
///
/// # Examples
///
/// ```
/// use linq_rs::*;
/// #[linq_rs::table]
/// struct User {
///     #[column("id_")]
///     #[primary]
///     id: usize,
///     first_name: String,
///     last_name: String,
///     #[cascade(from=col_id to=col_user_id)]
///     cards: Card,
///     created_time: DateTime,
///     updated_time: DateTime,
/// }
///
/// #[table]
/// struct Card {
///     #[primary]
///     id: usize,
///     user_id: usize,
///     card_no: String,
/// }
/// ```
pub use linq_proc_macro::table;
