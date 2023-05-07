pub type Block = Vec<Stmts>; // we can also have expressions at block level btw

#[derive(Debug, Clone)]
pub enum Stmts {
    Conditional {
        expr: Expr,
        true_block: Block,
        false_block: Option<Block>,
    },
    Expr {
        expr: Expr,
    },
}

#[derive(Debug, Clone)]
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
        is_static: bool,
        base: Box<Expr>,
        ident: Box<Expr>,
    },
    FunctionCall {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum BiOpOperation {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Modulo(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone)]
pub struct TypeReference {
    // Identifier, Index
    // (both can be generic)
    selector: Expr,
}

#[derive(Debug, Clone)]
pub enum Node {
    ImportStatement {
        lib: String,
    },
    FunctionDeclaration {
        name: String,
        args: Vec<(String, String)>,
        body: Vec<Stmts>,
    },
    VariableDeclaration {
        typ: TypeReference,
        name: String,
        val: Expr,
    },
    ClassDeclaration {
        name: String,
        funcs: Vec<Node>,
        vars: Vec<Node>,
    },
}
