use syn::{Expr, Lit};

pub enum Value {
    Lit(Lit),
    Expr(Expr),
}
