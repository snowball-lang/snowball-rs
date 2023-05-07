mod frontend;
mod backend;
use clap::Parser;
use frontend::parser;

#[macro_use]
extern crate lalrpop_util;

#[derive(Parser)]
struct Args {}

fn main() {
    println!("{:?}", parser::parse(String::from("fn main(arg: i8)")));
}
