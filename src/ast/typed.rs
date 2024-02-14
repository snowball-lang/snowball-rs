
use crate::ast::nodes;
use crate::ast::nodes::AST;
use crate::ast::attrs::AttrHandler;

#[derive(Debug, Clone)]
pub struct TypedNode {
    kind: Box<AST<TypedNode, TypedExprNode>>,
    attrs: Option<AttrHandler>,
}

#[derive(Debug, Clone)]
pub struct TypedExprNode {
    kind: Box<AST<TypedExprNode>>,
    attrs: Option<AttrHandler>,
}
