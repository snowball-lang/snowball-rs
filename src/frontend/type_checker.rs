use std::collections::HashMap;

use crate::ast::nodes::AST;
use crate::ast::typed::TypedNode;

pub enum UnificationType {
    Literal(LiteralTypes),
    Known(Type),
    TypeVariable(usize),
    Generic(String)
}

pub enum Type {
    Integer {
        size: usize,
        signed: bool
    },
    Float {
        size: usize
    }
}

impl UnificationType {
    pub fn try_coerce(&self, to: UnificationType) -> Option<UnificationType> {
        if let UnificationType::Literal(l) = self {
            
        }
    }
}

impl PartialEq<Type> for LiteralTypes {
    fn eq(&self, other: &Type) -> bool {
        match self {
            LiteralTypes::Float => {
                match other {
                    Type::Float { .. } => true,
                    _ => false
                }
            }
            LiteralTypes::Integer => {
                match other {
                    Type::Integer { .. } => true,
                    _ => false
                }
            }
        }
    }
}

pub enum LiteralTypes {
    Integer,
    Float,
}

pub fn typecheck(ast: Vec<AST>) -> Vec<AST> {
    Vec::new()
}

struct Typechecker {
    types: HashMap<usize, UnificationType> // TODO: Figure out changing it to a vec
}

impl Typechecker {
    pub fn new() -> Typechecker {
        Typechecker {
            types: HashMap::new()
        }
    }

    pub fn typecheck(ast: Vec<AST>) -> Vec<AST> {
        
    }

    pub fn infer_expr(&mut self) -> UnificationType {

    }

    pub fn unify(&mut self, lhs: UnificationType, rhs: UnificationType) {
        
    }
}