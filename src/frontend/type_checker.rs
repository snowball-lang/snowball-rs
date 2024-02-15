use std::collections::HashMap;

use crate::ast::nodes::AST;
use crate::ast::typed::TypedNode;

use crate::ast::nodes::Node;
use crate::frontend::module::{Module, NamespacePath};
use crate::reports::{CompileError, ErrorInfo, Error, Reports};
use crate::ast::nodes::AstType;

#[derive(Clone)]
pub enum UnificationType {
    Known(Type),
    TypeVariable(usize),
    Generic(String)
}

#[derive(Clone)]
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
    Void
}

#[derive(Debug, Clone)]
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
    types: HashMap<String, UnificationType>, // TODO: Figure out changing it to a vec
    functions: Vec<(NamespacePath, FunctionSymbol)>,
    scope: Vec<HashMap<String, Symbol>>,
    reports: Reports
}

macro_rules! report {
    ($self:ident, $error_type:expr, $node:expr) => { {
        $self.reports.add_error(CompileError::new($error_type, $node.get_location().unwrap().clone()));
        return Err(());
    }
    };
    ($self:ident, $error_type:expr, $node:expr, $info:expr) => {{
        $self.reports.add_error(CompileError::new($error_type, $node.get_location().unwrap().clone()).with_info($info));
        return Err(());
    }
    }
}

#[derive(Clone)]
pub enum Symbol {
    Variable(UnificationType),
    Function(FunctionSymbol),
    Type(UnificationType)
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
        self.initialize_builtin_types();
        self.run_checks(module.clone(), &mut new_top);
        new_module.set_top(AST::TopLevel(new_top));
        new_module
    }

    pub fn initialize_builtin_types(&mut self) {
        self.types.insert("i32".to_string(), UnificationType::Known(Type::Integer { size: 32, signed: true }));
        self.types.insert("i64".to_string(), UnificationType::Known(Type::Integer { size: 64, signed: true }));
        self.types.insert("f32".to_string(), UnificationType::Known(Type::Float { size: 32 }));
        self.types.insert("f64".to_string(), UnificationType::Known(Type::Float { size: 64 }));
        self.types.insert("u32".to_string(), UnificationType::Known(Type::Integer { size: 32, signed: false }));
        self.types.insert("u64".to_string(), UnificationType::Known(Type::Integer { size: 64, signed: false }));
        self.types.insert("void".to_string(), UnificationType::Known(Type::Void));
    }

    pub fn run_checks(&mut self, module: Module<Node>, new_node: &mut Vec<TypedNode>) -> Result<(), ()> {
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
                self.check_node(node, new_node)?;
            }
        }
        println!("{:#?}", self.functions);
        Ok(())
    }

    pub fn check_node(&mut self, node: Node, new_node: &mut Vec<TypedNode>) -> Result<(), ()> {
        match node.get_kind() {
            AST::FuncDef( name, args, ret, body, generics ) => {
                self.add_scope();
                if let Some(generics) = generics {
                    for generic in generics {
                        self.scope.last_mut().unwrap().insert(generic.get_name().clone(), Symbol::Type(UnificationType::Generic(generic.get_name().clone())));
                    }
                }
                for (name, ty) in args {
                    let ty = self.get_type(ty.clone())?;
                    self.scope.last_mut().unwrap().insert(name.clone(), Symbol::Variable(ty));
                }
                let ret = self.get_type(ret.clone())?;
                self.remove_scope();
            }
            _ => {}
        }
        Ok(())
    }

    pub fn get_type(&mut self, ty: AstType) -> Result<UnificationType, ()> {
        let symbol = self.get_symbol(ty.get_ast().clone())?;
        match symbol {
            Symbol::Variable( .. ) => report!(self, Error::UnexpectedItem("type".to_string(), "variable".to_string()), ty.get_ast().clone(), ErrorInfo {
                info: Some("This does not point towards a type.".to_string()),
                help: Some("Make sure there is no conflict between variable and type names.".to_string()),
                note: Some("Variables cant be used as types. Only types can be used as types.".to_string()),
                ..Default::default()
            }),
            Symbol::Function( .. ) => report!(self, Error::UnexpectedItem("type".to_string(), "function".to_string()), ty.get_ast().clone(), ErrorInfo {
                info: Some("This does not point towards a type.".to_string()),
                help: Some("Make sure there is no conflict between function and type names.".to_string()),
                note: Some("Functions cant be used as types. Only types can be used as types.".to_string()),
                ..Default::default()
            }),
            Symbol::Type( ty ) => Ok(ty.clone())
        }
    }
                
    pub fn get_symbol(&mut self, ty: Node) -> Result<Symbol, ()> {
        match ty.get_kind() {
            AST::Ident( name, _ ) => {
                let s = self.lookup_variable(&name);
                match s {
                    Some( sym ) => Ok(sym.clone()),
                    None => {
                        report!(self, Error::UnknownVariable(name.clone()), ty, ErrorInfo {
                            info: Some(format!("Variable '{}' not found!", name).to_string()),
                            help: Some("Make sure the variable is declared in the current scope.".to_string()),
                            ..Default::default()
                        });
                    }
                }
            }
            _ => {
                report!(self, Error::UnknownVariable("<todo>".to_string()), ty);
            }
        }
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

    fn lookup_variable(&self, var_name: &str) -> Option<Symbol> {
        for scope in self.scope.iter().rev() {
            if let Some(var_type) = scope.get(var_name) {
                return Some(var_type.clone());
            }
        }
        if let Some(var_type) = self.types.get(var_name) {
            return Some(Symbol::Type(var_type.clone()));
        }
        // TODO: Functions
        None
    }

    pub fn add_scope(&mut self) {
        self.scope.push(HashMap::new());
    }

    pub fn remove_scope(&mut self) {
        self.scope.pop();
    }
}