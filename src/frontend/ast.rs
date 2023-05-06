pub enum Expr {
    Num {
        val: i64,
    },
    BiOp {
        lhs: Box<Expr>,
        op: BiOpOperation,
        rhs: Box<Expr>,
    },
}

pub enum BiOpOperation {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Modulo(Box<Expr>, Box<Expr>),
}