use std::collections::HashMap;
use std::option::Option;
use std::vec::Vec;
use crate::ast::source::SourceLocation;
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
    Index,
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
pub struct GenericDecl<T: std::fmt::Debug + Clone = AstType> {
    name: String,
    default: Option<T>,
    impls: Vec<T>,
}

impl<T: std::fmt::Debug + Clone> GenericDecl<T> {
    pub fn new(name: String, impls: Vec<T>, default: Option<T>) -> Self {
        GenericDecl { name, impls, default }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_impls(&self) -> &Vec<T> {
        &self.impls
    }

    pub fn get_default(&self) -> Option<&T> {
        self.default.as_ref()
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
pub enum AST<T: std::fmt::Debug + Clone = Node, TN: std::fmt::Debug + Clone = AstType> {
    TopLevel(Vec<T>),
    Return(Option<T>),
    Break,
    Continue,
    If(T, T, Vec<T>),
    While(T, T, /* is_do_while */ bool),
    For(T, T, T, Vec<T>),
    Block(Vec<T>),
    FuncDef(/* name */ String, /* args */ HashMap<String, TN>, /* ret arg */TN, Option<T>, Option<Vec<GenericDecl<TN>>>),
    VarDef(String, Option<TN>, Option<T>),
    ClassDef(Option<T>, Vec<ClassMember>, Vec<GenericDecl<TN>>),
    NamespaceDef(Option<T>, Vec<T>),
    Import(T),
    InterfaceDef(Option<T>, Vec<T>, Vec<GenericDecl<TN>>),
    EnumDef(Option<T>, Vec<T>),
    Empty,
    ClassInit(TN, Vec<T>),
    ClassAccess(T, String),
    NamespaceAccess(T, String),
    Ident(String, Option<Vec<TN>>),
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Call(T, Vec<T>),
    Cast(T, TN),
    BinaryOp(BinaryOp, T, T, /* is_unary */ bool),
    Assign(T, T),
}

#[derive(Debug, Clone)]
pub struct Node {
    kind: Box<AST>,
    attrs: Option<AttrHandler>,
    location: Option<SourceLocation>,
}

impl Node {
    pub fn new(kind: AST) -> Self {
        Node { kind: Box::new(kind), attrs: None, location: None }
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

    pub fn with_location(&mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self.clone()
    }

    pub fn get_location(&self) -> Option<&SourceLocation> {
        self.location.as_ref()
    }
}
