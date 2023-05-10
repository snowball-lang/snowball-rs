use super::{lexer::Token, ast::Node};

struct TokBuf {
    pos: usize,
    toks: Vec<Token>,
}

impl TokBuf {
    pub fn new(toks: Vec<Token>) -> TokBuf {
        TokBuf { pos: 0, toks }
    } 

    pub fn advance(&mut self) {
        if self.in_bounds() {
            self.pos += 1;
            return;
        }
    }

    pub fn next(&mut self) -> Token {
        self.advance();
        self.current()
    }

    pub fn current(&self) -> Token {
        self.toks[self.pos].clone()
    }

    pub fn in_bounds(&self) -> bool {
        self.pos < self.toks.len()
    }
}

pub fn parse(toks: Vec<Token>) -> Vec<Node> {
    let mut buf = TokBuf::new(toks);
    let mut ret = Vec::new();
    
    ret
}