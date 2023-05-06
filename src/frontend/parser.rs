use crate::lalrpop_util::lalrpop_mod;

use super::ast::Node;
lalrpop_mod!(pub snowball);

pub fn parse(code: String) -> Vec<Node> {
    
}