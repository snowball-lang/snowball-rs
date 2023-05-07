mod backend;
mod frontend;
use std::fs;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    input: String,
    #[clap(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();
    let _code = fs::read_to_string(args.input).unwrap();
}
