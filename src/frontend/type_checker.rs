use std::collections::HashMap;

use crate::ast::nodes::AST;
use crate::ast::typed::TypedNode;

use crate::ast::nodes::Node;
use crate::frontend::module::{Module, NamespacePath};
use crate::reports::{CompileError, ErrorInfo, ErrorType, Reports};

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
    },
    Object {
        id: usize
    },
    Function {
        args: Vec<UnificationType>,
        ret: Box<UnificationType>
    },
    Pointer {
        ty: Box<UnificationType>
    },
    Reference {
        ty: Box<UnificationType>
    },
}

impl UnificationType {
    pub fn try_coerce(&self, to: UnificationType) -> Option<UnificationType> {
        if let UnificationType::Literal(l) = self {
            if l == &LiteralTypes::Integer {
                match to {
                    UnificationType::Known(Type::Integer { .. }) => return Some(to),
                    UnificationType::Literal(LiteralTypes::Integer) => return Some(to),
                    _ => return None
                }
            } else if l == &LiteralTypes::Float {
                match to {
                    UnificationType::Known(Type::Float { .. }) => return Some(to),
                    UnificationType::Literal(LiteralTypes::Float) => return Some(to),
                    _ => return None
                }
            }
        }
        None
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

#[derive(PartialEq)]
pub enum LiteralTypes {
    Integer,
    Float,
}

#[derive(Debug)]
pub struct FunctionSymbol {
    ast: Node
}

impl FunctionSymbol {
    pub fn new(ast: Node) -> FunctionSymbol {
        FunctionSymbol {
            ast
        }
    }

    pub fn get_ast(&self) -> &Node {
        &self.ast
    }
}

pub struct Typechecker {
    types: HashMap<usize, UnificationType>, // TODO: Figure out changing it to a vec
    functions: Vec<(NamespacePath, FunctionSymbol)>,
    scope: Vec<HashMap<String, UnificationType>>,
    reports: Reports
}

macro_rules! report {
    ($self:ident, $error_type:expr, $node:expr) => { {
        $self.reports.add_error(CompileError::new($error_type, $node.get_location().unwrap()));
        return Err(());
    }
    };
    ($self:ident, $error_type:expr, $node:expr, $info:expr) => {{
        $self.reports.add_error(CompileError::new($error_type, $node.get_location().unwrap()).with_info($info));
        return Err(());
    }
    }
}

impl Typechecker {
    pub fn new() -> Typechecker {
        Typechecker {
            types: HashMap::new(),
            scope: vec![HashMap::new()],
            reports: Reports::new(),
            functions: Vec::new()
        }
    }

    pub fn typecheck(&mut self, module: Module<Node>) -> Module<TypedNode> {
        let mut new_module = Module::<TypedNode>::new(module.get_path().clone(), module.get_file_name().clone());
        let mut new_top = Vec::new();
        self.run_checks(module.clone(), &mut new_top);
        new_module.set_top(AST::TopLevel(new_top));
        new_module
    }

    pub fn initialize_builtin_types(&mut self) {
        self.types.insert(0, UnificationType::Literal(LiteralTypes::Integer));
        self.types.insert(1, UnificationType::Literal(LiteralTypes::Float));
        self.types.insert(5, UnificationType::Known(Type::Integer { size: 1, signed: true }));
        self.types.insert(5, UnificationType::Known(Type::Integer { size: 8, signed: true }));
        self.types.insert(2, UnificationType::Known(Type::Integer { size: 32, signed: true }));
        self.types.insert(4, UnificationType::Known(Type::Integer { size: 64, signed: true }));
        self.types.insert(7, UnificationType::Known(Type::Integer { size: 8, signed: false }));
        self.types.insert(8, UnificationType::Known(Type::Integer { size: 16, signed: false }));
        self.types.insert(9, UnificationType::Known(Type::Integer { size: 32, signed: false }));
        self.types.insert(3, UnificationType::Known(Type::Float { size: 32 }));
    }

    pub fn run_checks(&mut self, module: Module<Node>, new_node: &mut Vec<TypedNode>) {
        // collect all function definitions
        if let AST::TopLevel (nodes) = module.get_top().clone() {
            for node in nodes {
                match node.get_kind() {
                    AST::FuncDef( name, .. ) => {
                        self.functions.push((Self::get_path_for_name(module.clone(), name.clone(), None), FunctionSymbol::new(node)));
                    }
                    _ => {}
                }
            }
        } else {
            panic!("Expected TopLevel node");
        }
        if let AST::TopLevel (nodes) = module.get_top().clone() {
            for node in nodes {
                //self.check_node(node, new_node);
            }
        }
        println!("{:#?}", self.functions);
    }

    pub fn get_path_for_name(module: Module<Node>, name: String, class_path: Option<NamespacePath>) -> NamespacePath {
        let mut path = module.get_path().clone();
        if let Some(class_path) = class_path {
            path.push_path(class_path);
        }
        path.push(name);
        path
    }

    pub fn get_reports(&self) -> &Reports {
        &self.reports
    }

    fn lookup_variable(&self, var_name: &str) -> Option<&UnificationType> {
        for scope in self.scope.iter().rev() {
            if let Some(var_type) = scope.get(var_name) {
                return Some(var_type);
            }
        }
        None
    }

    pub fn add_scope(&mut self) {
        self.scope.push(HashMap::new());
    }

    pub fn remove_scope(&mut self) {
        self.scope.pop();
    }
}