mod frontend;
mod backend;
use std::fs;

use clap::Parser;
use frontend::parser;


#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    input: String,
    #[clap(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();
    let code = fs::read_to_string(args.input).unwrap();
    
}
