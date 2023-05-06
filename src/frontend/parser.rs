pub enum Expr {
    Num { val: i64 },
    BiOp { lhs: Box<Expr>, op: BiOpOperation, rhs: Box<Expr> },
}

pub enum BiOpOperation {
    Add,
    Sub,
    Mul,
    Div,
    Modulo
}