
use crate::ast::nodes;
use crate::ast::nodes::AST;
use crate::ast::attrs::AttrHandler;

#[derive(Debug, Clone)]
pub struct TypedNode {
    kind: Box<AST<TypedNode>>,
    attrs: Option<AttrHandler>,
}