#[derive(Debug, PartialEq)]
pub enum Variant<C, E> {
    Constant(C),

    Eval(E),
}
