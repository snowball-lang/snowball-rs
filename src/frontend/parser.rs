use std::process::exit;

use ariadne::{Label, Report, ReportKind, Source};

use super::{
    ast::{Node, TypeReference},
    lexer::{TokType, Token},
};

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

    pub fn consume(&mut self, tok: Vec<TokType>) -> Option<Token> {
        self.advance();
        let mut was_ret = false;
        if self.in_bounds() {
            for item in tok {
                if item == self.current().tok {
                    was_ret = true;
                }
            }
        } else {
            panic!("called consume with not enough buffer space")
        }
        if was_ret {
            return Some(self.current());
        } else {
            return None;
        }
    }

    pub fn get_tok_idx_span(&self, idx: usize) -> (usize, usize) {
        (self.toks[idx].start, self.toks[idx].end)
    }

    pub fn get_tok_idx_line(&self, idx: usize) -> usize {
        self.toks[idx].line
    }

    pub fn get_current_tok_idx(&self) -> usize {
        self.pos
    }

    pub fn get_cur_tok_line(&self) -> usize {
        self.get_tok_idx_line(self.pos)
    }

    pub fn get_cur_tok_span(&self) -> (usize, usize) {
        self.get_tok_idx_span(self.pos)
    }
}

pub struct Parser {
    buf: TokBuf,
    prog: Vec<Node>,
    src: String,
}

impl Parser {
    pub fn new(prog: Vec<Token>, src: String) -> Parser {
        Parser {
            buf: TokBuf::new(prog),
            prog: Vec::new(),
            src,
        }
    }
    pub fn parse(&mut self) -> Vec<Node> {
        let mut ret = Vec::new();
        while self.buf.in_bounds() {
            match self.buf.current().tok {
                TokType::KWordFunc => {
                    // from: https://github.com/snowball-lang/snowball/blob/dev/src/parser/parseFunction.cc
                    let name = self
                        .buf
                        .consume(vec![TokType::Ident])
                        .expect("expected Ident");
                    self.buf
                        .consume(vec![TokType::BracketLParen])
                        .expect("expected LParen");
                    self.arg_parser();
                }
                _ => todo!("unimplimented tok type: {:?}", self.buf.current().tok),
            }
            self.buf.advance();
        }
        ret
    }

    //fn parse_type(&mut self) -> TypeReference {
    //
    //}

    fn arg_parser(&mut self) -> Vec<(String, String)> {
        let mut ret = Vec::new();
        loop {
            let arg_name = self.consume_and_err(
                vec![TokType::Ident],
                "expected identifier for argument name",
                self.buf.get_cur_tok_line(),
                vec![((self.buf.get_cur_tok_span()), String::from("here"))],
            );
            let arg_type = self.consume_and_err(
                vec![TokType::Ident],
                "expected type after argument name",
                self.buf.get_cur_tok_line(),
                vec![((self.buf.get_cur_tok_span()), String::from("here"))],
            );
            ret.push((arg_name.val, arg_type.val));
            match self.buf.consume(vec![TokType::SymbolComma]) {
                Some(_) => self.buf.advance(),
                None => break,
            }
        }

        ret
    }

    fn consume_and_err(
        &mut self,
        tok: Vec<TokType>,
        error: &str,
        lineno: usize,
        arrows: Vec<((usize, usize), String)>,
    ) -> Token {
        match self.buf.consume(tok) {
            Some(t) => t,
            None => {
                self.error(error.to_string(), lineno, arrows);
                exit(-1);
            }
        }
    }

    fn error(&self, error: String, lineno: usize, arrows: Vec<((usize, usize), String)>) {
        let mut labels = Vec::new();
        for arrow in arrows {
            labels.push(Label::new(arrow.0 .0..arrow.0 .1).with_message(arrow.1));
        }
        Report::build(ReportKind::Error, (), lineno)
            .with_message(error)
            .with_labels(labels)
            .finish()
            .print(Source::from(self.src.clone()))
            .unwrap();
    }
}
