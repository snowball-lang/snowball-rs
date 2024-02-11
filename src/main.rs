use clap::Parser as ClapParser;

mod ast;
mod compiler;
mod frontend;
mod reports;
mod utils;

#[derive(ClapParser)]
struct Args {
    input: String,
}

fn main() {
    let args = Args::parse();

    let compiler = crate::compiler::Compiler::new(args.input);
    compiler.run();
}
