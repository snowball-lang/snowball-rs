

pub mod token;

use crate::{ast::source::SourceLocation, compiler::Compiler};
use crate::reports::{CompileError, Reports, Error};

pub struct Lexer {
  input: String,
  read_position: usize,
  location: SourceLocation,
  reports: Reports,
}

impl Lexer {
  pub fn new(content: String, path: String) -> Lexer {
    Lexer {
      input: content,
      read_position: 0,
      location: SourceLocation::new(path, 0, 0, 0),
      reports: Reports::new(),
    }
  }

  pub fn get_reports(self) -> Reports {
    self.reports
  }

  pub fn lex(&mut self) {
    for c in self.input.chars() {
      match c {
        _ => {
          self.reports.add_error(CompileError::new(
            Error::UnexpectedChar(c),
            self.location.clone(),
          ));
        }
      }
    }
  }
}