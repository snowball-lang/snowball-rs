pub type Block = Vec<Stmts>; // we can also have expressions at block level btw

pub enum Stmts {
    Conditional {
        expr: Box<Expr>,
        TrueBlock: Box<Block>,
        FalseBlock: Option<Box<Block>>,
    },
    Expr {
        expr: Expr,
    },
}

pub enum Expr {
    Num {
        val: i32,
    },
    BiOp {
        lhs: Box<Expr>,
        op: BiOpOperation,
        rhs: Box<Expr>,
    },
    Identifier {
        name: String,
        generics: Vec<TypeReference>,
    },
    Index {
        isStatic: bool,
        base: Box<Expr>,
        ident: Box<Expr>,
    },
    FunctionCall {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },
}

pub enum BiOpOperation {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Modulo(Box<Expr>, Box<Expr>),
}

pub struct TypeReference {
    // Identifier, Index
    // (both can be generic)
    selector: Expr,
}

pub enum Node {
    ImportStatement(String),
    FunctionDeclaration(String, Vec<Box<Stmts>>),
    VariableDeclaration(TypeReference, String, Expr),
    ClassDeclaration(
        String,
        /* functions = */ Vec<Box<Node>>,
        /* variables = */ Vec<Box<Node>>,
    ),
}
