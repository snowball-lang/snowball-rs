use std::process::exit;

use crate::lalrpop_util::lalrpop_mod;

use super::ast::Node;
lalrpop_mod!(pub snowball);

pub fn parse(code: String) -> Vec<Node> {
    let parser = snowball::SnowballParser::new().parse("use Core::IO");
    match parser {
        Ok(ast) => ast,
        Err(e) => { println!("{}", e); exit(-1) }
    }
}