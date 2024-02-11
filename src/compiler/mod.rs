use std::fs;

pub fn default_file_loader(path: String) -> String {
    fs::read_to_string(path).unwrap()
}

pub static mut file_loader: fn(String) -> String = default_file_loader;

pub struct Compiler {
    path: String,
}

impl Compiler {
    pub fn new(path: String) -> Compiler {
        Compiler { path }
    }

    pub fn run(&self) {
        let source;
        unsafe {
            source = (file_loader)(self.path.clone());
        }

        // TODO: Iterate through the folder but for now, we just get the file
        let mut lexer = crate::frontend::lexer::Lexer::new(source, self.path.clone());
        lexer.lex();

        if lexer.get_reports().handle_errors() {
            return;
        }

        let tokens = lexer.get_tokens();
        println!("{:?}", tokens);
    }
}

pub fn set_file_loader(new: fn(String) -> String) {
    unsafe {
        file_loader = new;
    }
}
