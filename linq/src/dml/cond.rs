use crate::variant::Variant;

#[derive(Debug, Clone, PartialEq)]
pub enum CondOp {
    NotEq,
    Eq,
    Gt,
    Lt,
    Gte,
    Lte,
    Like,
    In,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CondParam {
    VariantList(Vec<Variant>),
    Variant(Variant),
    CondExpr(Box<CondExpr>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CondExpr {
    pub op: CondOp,
    pub lhs: CondParam,
    pub rhs: CondParam,
}
