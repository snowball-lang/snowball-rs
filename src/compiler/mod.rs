
use std::fs;

pub fn default_file_loader(path: String) -> String {
    fs::read_to_string(path).unwrap()
}

pub struct Compiler {
    file_loader: fn(String) -> String,
    path: String,
}

impl Compiler {
    pub fn new(path: String) -> Compiler {
        Compiler {
            file_loader: default_file_loader,
            path,
        }
    }

    pub fn run(&self) {
        let source = (self.file_loader)(self.path.clone());
        
        // TODO: Iterate through the folder but for now, we just get the file
        let mut lexer = crate::frontend::lexer::Lexer::new(source, self.path.clone());
        lexer.lex();

        if lexer.get_reports().handle_errors() {
            return;
        }

        let tokens = lexer.get_tokens();
        println!("{:?}", tokens);
    }

    pub fn set_file_loader(&mut self, file_loader: fn(String) -> String) {
        self.file_loader = file_loader;
    }
}
