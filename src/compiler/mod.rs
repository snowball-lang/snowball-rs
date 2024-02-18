use std::fs;

use crate::frontend::module::NamespacePath;

pub struct Compiler {
    path: String,
}

impl Compiler {
    pub fn new(path: String) -> Compiler {
        Compiler { path }
    }

    pub fn run(&self) {
        let source = fs::read_to_string(self.path.clone()).expect("Something went wrong reading the file");

        // TODO: Iterate through the folder but for now, we just get the file
        let mut lexer = crate::frontend::lexer::Lexer::new(source, self.path.clone());
        lexer.lex();
        if lexer.get_reports().handle_errors() {
            return;
        }
        let mut parser = crate::frontend::parser::Parser::new(NamespacePath::from_path(self.path.clone()), self.path.clone(), &lexer);
        let result = &parser.parse();
        if parser.get_reports().handle_errors() {
            return;
        }
        let mut typechecker = crate::frontend::type_checker::Typechecker::new();
        let typed_module = typechecker.typecheck(&mut (result.clone().unwrap()));
        if typechecker.get_reports().handle_errors() {
            return;
        }
    }
}
