use std::option::Option;
use std::vec::Vec;

pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    New,
    Del,
}

pub struct AstType {
    ast: Node,
}

impl AstType {
    pub fn new(ast: Node) -> Self {
        AstType { ast }
    }

    pub fn get_ast(&self) -> &Node {
        &self.ast
    }
}

pub struct GenericDecl {
    name: String,
    impls: Vec<AstType>,
}

impl GenericDecl {
    pub fn new(name: String, impls: Vec<AstType>) -> Self {
        GenericDecl { name, impls }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_impls(&self) -> &Vec<AstType> {
        &self.impls
    }
}

pub struct ClassMember {
    name: String,
    ty: AstType,
}

impl ClassMember {
    pub fn new(name: String, ty: AstType) -> Self {
        ClassMember { name, ty }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_ty(&self) -> &AstType {
        &self.ty
    }
}

pub enum AST {
    Return(Option<Node>),
    Break,
    Continue,
    If(Node, Node, Vec<Node>),
    While(Node, Vec<Node>, /* is_do_while */ bool),
    For(Node, Node, Node, Vec<Node>),
    Block(Vec<Node>),
    Assign(Node, Node),
    Call(Node, Vec<Node>),
    Cast(Node, AstType),
    BinaryOp(BinaryOp, Node, Node, /* is_unary */ bool),
    FuncDef(Option<Node>, Vec<Node>, Vec<Node>, Vec<GenericDecl>),
    VarDef(Option<Node>, Node),
    Ident(String, Vec<AstType>),
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    ClassDef(Option<Node>, Vec<ClassMember>, Vec<GenericDecl>),
    ClassInit(AstType, Vec<Node>),
    ClassAccess(Node, String),
    NamespaceDef(Option<Node>, Vec<Node>),
    NamespaceAccess(Node, String),
    Import(Node),
    InterfaceDef(Option<Node>, Vec<Node>, Vec<GenericDecl>),
    EnumDef(Option<Node>, Vec<Node>),
}

pub struct Node {
    kind: AST,
}

impl Node {
    pub fn new(kind: AST) -> Self {
        Node { kind }
    }

    pub fn get_kind(&self) -> &AST {
        &self.kind
    }
}
