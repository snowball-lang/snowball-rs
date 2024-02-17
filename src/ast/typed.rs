
use crate::ast::nodes;
use crate::ast::nodes::AST;
use crate::ast::attrs::AttrHandler;
use crate::frontend::type_checker::UnificationType;

#[derive(Debug, Clone)]
pub struct TypedNode {
    kind: Box<AST<TypedNode, UnificationType>>,
    attrs: Option<AttrHandler>,
}

impl TypedNode {
    pub fn new(kind: AST<TypedNode, UnificationType>, attrs: Option<AttrHandler>) -> Self {
        TypedNode { kind: Box::new(kind), attrs }
    }

    pub fn get_kind(&self) -> &Box<AST<TypedNode, UnificationType>> {
        &self.kind
    }

    pub fn get_attrs(&self) -> &Option<AttrHandler> {
        &self.attrs
    }
}
