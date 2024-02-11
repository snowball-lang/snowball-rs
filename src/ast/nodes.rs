use std::collections::HashMap;
use std::option::Option;
use std::vec::Vec;
use crate::ast::attrs::AttrHandler;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
    FuncDef(/* name */ String, /* args */ HashMap<String, AstType>, /* ret arg */AstType, Option<Vec<Node>>, Option<Vec<GenericDecl>>),
    VarDef(Option<Node>, Node),
    Ident(String, Option<Vec<AstType>>),
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

#[derive(Debug, Clone)]
pub struct Node {
    kind: Box<AST>,
    attrs: Option<AttrHandler>,
}

impl Node {
    pub fn new(kind: AST) -> Self {
        Node { kind: Box::new(kind), attrs: None }
    }

    pub fn get_kind(&self) -> &AST {
        &self.kind
    }

    pub fn with_attrs(&mut self, attrs: AttrHandler) -> &Self {
        self.attrs = Some(attrs);
        self
    }

    pub fn get_attrs(&self) -> Option<&AttrHandler> {
        self.attrs.as_ref()
    }
}
