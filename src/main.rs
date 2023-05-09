mod backend;
mod frontend;

use std::fs;

use clap::Parser;
use frontend::{lexer::tokenise, parser};

#[derive(Parser)]
struct Args {
    input: String,
    #[clap(short, default_value = "out.o")]
    output: String,
}

fn main() {
    let args = Args::parse();
    let code = fs::read_to_string(args.input.clone()).unwrap();
    let tokens = tokenise(code);
    println!("{:#?}", tokens);
}
