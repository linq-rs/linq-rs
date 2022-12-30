mod variant;
pub use variant::*;

/// This module define LINQ ir code executor traits.
pub mod driver;

/// This module define LINQ IR structures for SQL [`DML`](https://www.javatpoint.com/dbms-sql-command) clause
pub mod dml;

/// This module define LINQ IR structures for SQL [`DDL`](https://www.javatpoint.com/dbms-sql-command) clause
pub mod ddl;

/// Proc macros to build LINQ language ir data.
pub use linq_proc_macro::*;

/// LINQ to support orm api.
pub mod orm;

pub use anyhow;
