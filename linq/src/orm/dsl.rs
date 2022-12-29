use crate::dml::CondExpr;

pub trait Where {
    type Context;
    fn cond(self, cond: CondExpr) -> Self::Context;
}

pub trait Limit {
    type Context;
    fn limit(self, count: usize) -> Self::Context;
}

pub trait Offset {
    type Context;
    fn offset(self, offset: usize) -> Self::Context;
}

pub trait Order<'a> {
    type Context;
    fn order_by(self, col_name: &'a str, desc: bool) -> Self::Context;
}
