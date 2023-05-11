mod backend;
mod frontend;

use std::fs;

use clap::Parser as ClapParser;
use frontend::{lexer::tokenise, parser::Parser};

#[derive(ClapParser)]
struct Args {
    input: String,
    #[clap(short, default_value = "out.o")]
    output: String,
}

fn main() {
    let args = Args::parse();
    let code = fs::read_to_string(args.input.clone()).unwrap();
    let tokens = tokenise(code.clone());
    println!("{:#?}", tokens);
    let ast = Parser::new(tokens, code).parse();
}
