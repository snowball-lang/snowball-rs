pub mod token;

use crate::frontend::lexer::token::{Token, TokenType};
use crate::reports::{CompileError, Error, ErrorType, Reports};
use crate::{ast::source::SourceLocation, compiler::Compiler};

pub struct Lexer {
    input: Vec<u8>,
    read_position: usize,
    location: SourceLocation,
    reports: Reports,
    tokens: Vec<Token>,
}

enum ReadMode {
    Integer,
    Float,
    Binary,
    Octal,
    Hex,
}

impl Lexer {
    pub fn new(content: String, path: String) -> Lexer {
        Lexer {
            input: content.chars().map(|c| c as u8).collect(),
            read_position: 0,
            location: SourceLocation::new(path, 1, 0, 0),
            reports: Reports::new(),
            tokens: Vec::new(),
        }
    }

    pub fn get_reports(&self) -> &Reports {
        &self.reports
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    pub fn lex(&mut self) {
        while self.read_position < self.input.len() {
            self.handle_char();
        }
        self.append_token(TokenType::EOF, 0);
    }

    fn handle_char(&mut self) {
        match self.get_char(0) {
            ' ' => self.next_char(1),
            '\t' => self.next_char(1),
            '\n' => {
                self.next_char(1);
                self.next_line();
            }
            '\r' => self.next_char(1),
            '\0' => self.next_char(1),
            '/' => match self.get_char(1) {
                '/' => {
                    while self.get_char(0) != '\n' && self.get_char(0) != '\0' {
                        self.next_char(1);
                    }
                }
                '*' => {
                    self.next_char(1);
                    self.next_char(1);
                    while self.get_char(0) != '*' && self.get_char(1) != '/' {
                        self.next_char(1);
                        if self.get_char(0) == '\n' {
                            self.next_line();
                        } else if self.get_char(0) == '\0' {
                            self.report_error(Error::UnexpectedEOF);
                        }
                    }
                    self.next_char(1);
                    self.next_char(1);
                }
                '=' => self.consume(TokenType::SlashEqual, 2),
                _ => self.consume(TokenType::Slash, 0),
            },
            '+' => match self.get_char(1) {
                '+' => self.consume(TokenType::DoublePlus, 2),
                '=' => self.consume(TokenType::PlusEqual, 2),
                _ => self.consume(TokenType::Plus, 0),
            },
            '-' => match self.get_char(1) {
                '-' => self.consume(TokenType::DoubleMinus, 2),
                '=' => self.consume(TokenType::MinusEqual, 2),
                _ => self.consume(TokenType::Minus, 0),
            },
            '*' => match self.get_char(1) {
                '=' => self.consume(TokenType::StarEqual, 2),
                _ => self.consume(TokenType::Star, 0),
            },
            '%' => match self.get_char(1) {
                '=' => self.consume(TokenType::PercentEqual, 2),
                _ => self.consume(TokenType::Percent, 0),
            },
            '&' => match self.get_char(1) {
                '&' => self.consume(TokenType::DoubleAmpersand, 2),
                '=' => self.consume(TokenType::AmpersandEqual, 2),
                _ => self.consume(TokenType::Ampersand, 0),
            },
            '|' => match self.get_char(1) {
                '|' => self.consume(TokenType::DoublePipe, 2),
                '=' => self.consume(TokenType::PipeEqual, 2),
                _ => self.consume(TokenType::Pipe, 0),
            },
            '=' => match self.get_char(1) {
                '=' => self.consume(TokenType::DoubleEqual, 2),
                '>' => self.consume(TokenType::Arrow, 2),
                _ => self.consume(TokenType::Equal, 0),
            },
            '!' => match self.get_char(1) {
                '=' => self.consume(TokenType::NotEqual, 2),
                _ => self.consume(TokenType::Exclamation, 0),
            },
            '<' => match self.get_char(1) {
                '<' => match self.get_char(2) {
                    '=' => self.consume(TokenType::DoubleLessThanEqual, 3),
                    _ => self.consume(TokenType::DoubleLessThan, 2),
                },
                '=' => self.consume(TokenType::LessThanEqual, 2),
                _ => self.consume(TokenType::LessThan, 0),
            },
            '>' => match self.get_char(1) {
                '>' => match self.get_char(2) {
                    '=' => self.consume(TokenType::DoubleGreaterThanEqual, 3),
                    _ => self.consume(TokenType::DoubleGreaterThan, 2),
                },
                '=' => self.consume(TokenType::GreaterThanEqual, 2),
                _ => self.consume(TokenType::GreaterThan, 0),
            },
            '(' => self.consume(TokenType::OpenParen, 0),
            ')' => self.consume(TokenType::CloseParen, 0),
            '{' => self.consume(TokenType::OpenBrace, 0),
            '}' => self.consume(TokenType::CloseBrace, 0),
            '[' => self.consume(TokenType::OpenBracket, 0),
            ']' => self.consume(TokenType::CloseBracket, 0),
            ';' => self.consume(TokenType::Semicolon, 0),
            ':' => self.consume(TokenType::Colon, 0),
            ',' => self.consume(TokenType::Comma, 0),
            '.' => self.consume(TokenType::Dot, 0),
            '@' => self.consume(TokenType::At, 0),
            '?' => self.consume(TokenType::Question, 0),
            '~' => self.consume(TokenType::Tilde, 0),
            '#' => self.consume(TokenType::Hash, 0),
            '"' => self.lex_string(),
            '\'' => self.lex_char(),
            _ => {
                if self.get_char(0).is_digit(10) {
                    self.lex_number();
                    return;
                } else if self.get_char(0).is_alphabetic() || self.get_char(0) == '_' {
                    self.lex_identifier();
                    return;
                }
                self.report_error(Error::UnexpectedChar(self.get_char(0)));
            }
        }
    }

    fn lex_char(&mut self) {
        self.next_char(1);
        let loc = self.location.clone();
        let mut chr = self.get_char(0);
        if chr == '\\' {
            self.next_char(1);
            match self.get_char(0) {
                'n' => chr = '\n',
                'r' => chr = '\r',
                't' => chr = '\t',
                '0' => chr = '\0',
                '\\' => chr = '\\',
                '\'' => chr = '\'',
                '"' => chr = '"',
                _ => {
                    self.reports.add_error(CompileError::warning(
                        Error::UnknownEscapeSequence(self.get_char(0)),
                        self.location.with_width(2),
                    ));
                    chr = self.get_char(0);
                }
            }
        }
        self.next_char(1);
        if self.get_char(0) != '\'' {
            self.report_error(Error::UnexpectedChar(self.get_char(0)));
        }
        self.append_token(TokenType::Char(chr as u8), 3);
        self.next_char(1);
        self.tokens.last_mut().unwrap().set_location(loc.with_width(3));
    }

    fn lex_string(&mut self) {
        let pos = self.location.clone();
        self.next_char(1);
        let mut string = String::new();
        let col = self.location.column;
        let line = self.location.line;
        while self.get_char(0) != '"' {
            if self.get_char(0) == '\n' || self.get_char(0) == '\0' {
                self.report_error(Error::UnexpectedEOF);
            }
            if self.get_char(0) == '\\' {
                self.next_char(1);
                match self.get_char(0) {
                    'n' => string.push('\n'),
                    'r' => string.push('\r'),
                    't' => string.push('\t'),
                    '0' => string.push('\0'),
                    '\\' => string.push('\\'),
                    '"' => string.push('"'),
                    '\'' => string.push('\''),
                    '\n' => {
                        self.next_line();
                    }
                    _ => {
                        self.reports.add_error(CompileError::warning(
                            Error::UnknownEscapeSequence(self.get_char(0)),
                            self.location.with_width(2),
                        ));
                        string.push(self.get_char(0));
                    }
                }
            } else {
                string.push(self.get_char(0));
            }
            self.next_char(1);
        }
        self.append_token(TokenType::String(string), self.location.column - col + 1);
        self.next_char(1);
        self.tokens.last_mut().unwrap().set_location(pos.with_width(self.location.column - col + 1));
    }

    fn lex_identifier(&mut self) {
        let mut id = String::from(self.get_char(0));
        let loc = self.location.clone();
        self.next_char(1);
        while (self.get_char(0).is_alphabetic() || self.get_char(0) == '_')
            || self.get_char(0).is_digit(10)
        {
            id.push(self.get_char(0));
            self.next_char(1);
        }
        match &*id {
            "true" => self.append_token(TokenType::True, 4),
            "false" => self.append_token(TokenType::False, 5),
            "if" => self.append_token(TokenType::If, 2),
            "else" => self.append_token(TokenType::Else, 4),
            "while" => self.append_token(TokenType::While, 5),
            "for" => self.append_token(TokenType::For, 3),
            "return" => self.append_token(TokenType::Return, 6),
            "break" => self.append_token(TokenType::Break, 5),
            "continue" => self.append_token(TokenType::Continue, 8),
            "func" => self.append_token(TokenType::Fn, 2),
            "new" => self.append_token(TokenType::New, 3),
            "super" => self.append_token(TokenType::Super, 5),
            "do" => self.append_token(TokenType::Do, 2),
            "import" => self.append_token(TokenType::Import, 6),
            "let" => self.append_token(TokenType::Let, 3),
            "mut" => self.append_token(TokenType::Mut, 3),
            "struct" => self.append_token(TokenType::Struct, 6),
            "enum" => self.append_token(TokenType::Enum, 4),
            "class" => self.append_token(TokenType::Class, 5),
            "interface" => self.append_token(TokenType::Interface, 9),
            "public" => self.append_token(TokenType::Public, 6),
            "private" => self.append_token(TokenType::Private, 7),
            "const" => self.append_token(TokenType::Const, 5),
            "static" => self.append_token(TokenType::Static, 6),
            "inline" => self.append_token(TokenType::Inline, 6),
            "external" => self.append_token(TokenType::External, 8),
            "abstract" => self.append_token(TokenType::Abstract, 8),
            "final" => self.append_token(TokenType::Final, 5),
            "override" => self.append_token(TokenType::Override, 8),
            _ => self.append_token(TokenType::Identifier(id.clone()), id.len()),
        }
        self.tokens.last_mut().unwrap().set_location(loc.with_width(id.len()));
    }

    fn lex_number(&mut self) {
        let loc = self.location.clone();
        let mut read_mode = ReadMode::Integer;
        if self.get_char(0) == '0' {
            match self.get_char(1) {
                'b' => read_mode = ReadMode::Binary,
                'o' => read_mode = ReadMode::Octal,
                'x' => read_mode = ReadMode::Hex,
                _ => {}
            }
        }
        let mut is_range = false;
        let chr = self.get_char(0);
        let mut num = chr.to_string();
        self.next_char(1);
        match read_mode {
            ReadMode::Integer => {
                while self.get_char(0).is_digit(10) || self.get_char(0) == '.' {
                    if self.get_char(0) == '.' {
                        match read_mode {
                            ReadMode::Float => {
                                read_mode = ReadMode::Integer;
                                num.pop();
                                is_range = true;
                                break;
                            }
                            _ => {}
                        }
                        read_mode = ReadMode::Float;
                    }
                    num.push(self.get_char(0));
                    self.next_char(1);
                }
            }
            ReadMode::Binary => {
                num.push(self.get_char(0));
                self.next_char(1); // skip 'b'
                while self.get_char(0).is_digit(2) {
                    num.push(self.get_char(0));
                    self.next_char(1);
                }
            }
            ReadMode::Octal => {
                num.push(self.get_char(0));
                self.next_char(1); // skip 'o'
                while self.get_char(0).is_digit(8) {
                    num.push(self.get_char(0));
                    self.next_char(1);
                }
            }
            ReadMode::Hex => {
                num.push(self.get_char(0));
                self.next_char(1); // skip 'x'
                while self.get_char(0).is_digit(16) {
                    num.push(self.get_char(0));
                    self.next_char(1);
                }
            }
            _ => {}
        }
        let mut append_dot = false;
        if num.len() > 1 && num.chars().last().unwrap() == '.' {
            num.pop();
            append_dot = true;
            read_mode = ReadMode::Float;
        }
        match self.get_char(0) {
            'U' | 'u' | 'L' | 'l' | 'F' | 'f' | 'D' | 'd' => {
                num.push(self.get_char(0));
                self.next_char(1);
            }
            _ => {}
        }
        match read_mode {
            ReadMode::Float => self.append_token(TokenType::Float(num.clone()), num.len()),
            _ => self.append_token(TokenType::Integer(num.clone()), num.len()),
        }
        self.tokens.last_mut().unwrap().set_location(loc.with_width(num.len()));
        if is_range {
            self.consume(TokenType::Dot, 0);
            self.consume(TokenType::Dot, 0);
            self.read_position -= 1;
        }
        if append_dot {
            self.consume(TokenType::Dot, 0);
            self.read_position -= 1;
        }
    }

    fn next_line(&mut self) {
        self.location.line += 1;
        self.location.column = 0;
    }

    fn append_token(&mut self, token_type: TokenType, width: usize) {
        self.tokens
            .push(Token::new(token_type, self.location.with_width(width)));
    }

    fn get_char(&self, offset: usize) -> char {
        if self.read_position + offset >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position + offset] as char
        }
    }

    fn consume(&mut self, token_type: TokenType, mut width: usize) {
        if width == 0 {
            width = 1;
        }
        self.append_token(token_type, width);
        self.next_char(width);
    }

    fn next_char(&mut self, width: usize) {
        self.location.column += width;
        self.read_position += width;
    }

    fn report_error(&mut self, error: Error) {
        self.reports
            .add_error(CompileError::new(error, self.location.with_width(1)));
    }
}
