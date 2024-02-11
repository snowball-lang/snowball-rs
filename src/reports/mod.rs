use crate::{black, blue, bold, compiler::file_loader, red, reset, yellow};
use std::io::Write;

pub enum Error {
    UnexpectedChar(char),
    UnknownEscapeSequence(char),
    UnexpectedEOF,
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::UnexpectedChar(c) => format!("unexpected character: '{}'", c),
            Error::UnexpectedEOF => "unexpected end of file".to_string(),
            Error::UnknownEscapeSequence(c) => format!("unknown escape sequence: '\\{}'", c),
        }
    }
}

pub enum ErrorType {
    Error,
    Warning,
}

pub struct CompileError {
    error_type: ErrorType,
    message: Error,
    location: crate::ast::source::SourceLocation,

    help: Option<String>,
    note: Option<String>,
    info: Option<String>,
}

impl CompileError {
    pub fn new(error_type: Error, location: crate::ast::source::SourceLocation) -> CompileError {
        CompileError {
            error_type: ErrorType::Error,
            message: error_type,
            location,
            help: None,
            note: None,
            info: None,
        }
    }

    pub fn warning(
        error_type: Error,
        location: crate::ast::source::SourceLocation,
    ) -> CompileError {
        CompileError {
            error_type: ErrorType::Warning,
            message: error_type,
            location,
            help: None,
            note: None,
            info: None,
        }
    }

    pub fn with_help(mut self, help: String) -> CompileError {
        self.help = Some(help);
        self
    }

    pub fn with_note(mut self, note: String) -> CompileError {
        self.note = Some(note);
        self
    }

    pub fn with_info(mut self, info: String) -> CompileError {
        self.info = Some(info);
        self
    }

    /// @brief replace 'hello' with [white]'hello'[reset]
    /// @param str input string
    /// @return string with 'hello' replaced with [white]'hello'[reset]
    fn print_highlight(s: String) -> String {
        let mut result = String::new();
        let mut i = 0;
        let str: Vec<char> = s.chars().collect();
        let mut in_highlight = false;
        while i < s.len() {
            let mut add = false;
            if str[i] == '\'' {
                if in_highlight {
                    add = true;
                } else {
                    in_highlight = true;
                    result.push_str(bold!());
                }
            }
            result.push(str[i]);
            if add {
                result.push_str(reset!());
                in_highlight = false;
            }
            i += 1;
        }
        result
    }

    fn print(&self) {
        let prefix = match self.error_type {
            ErrorType::Error => format!("{}error", red!()),
            ErrorType::Warning => format!("{}warning", yellow!()),
        };
        let mut result = format!(
            "\n{}{}{}: {}{}\n at [{}{}{}:{}{}{}:{}{}]\n",
            bold!(),
            prefix,
            reset!(),
            Self::print_highlight(self.message.to_string()),
            bold!(),
            black!(),
            self.location.path,
            reset!(),
            bold!(),
            blue!(),
            self.location.line,
            self.location.column,
            reset!(),
        );

        let mut line: usize = 0;
        let file_content;
        unsafe {
            file_content = file_loader(self.location.path.clone());
        }
        result.push_str(format!("{}    |\n", black!()).as_str());
        for l in file_content.lines() {
            line += 1;
            if (line as isize >= (self.location.line as isize - 2))
                && (line <= self.location.line + 2)
            {
                if line == self.location.line {
                    let mut result_line = String::new();
                    for (i, c) in l.chars().enumerate() {
                        if i as isize == (self.location.column as isize - 1) {
                            result_line.push_str(reset!());
                            result_line.push_str(bold!());
                        }
                        result_line.push(c);
                        if i == (self.location.column + self.location.width - 1) {
                            result_line.push_str(reset!());
                            result_line.push_str(black!());
                        }
                    }
                    result.push_str(format!("{:3} | {}\n", line, result_line).as_str());
                    result.push_str(format!("{}    | ", black!()).as_str());
                    for i in 0..l.len() {
                        if i as isize == (self.location.column as isize - 1) {
                            result.push_str(reset!());
                            result.push_str(bold!());
                            result.push_str(red!());
                            for _ in 0..self.location.width {
                                result.push_str("^");
                            }
                            result.push_str(reset!());
                            break;
                        } else {
                            result.push_str(" ");
                        }
                    }
                    result.push_str("\n");
                } else {
                    result.push_str(&format!("{}{:3} | {}\n", black!(), line, l));
                }
            }
        }
        result.push_str(format!("{}    |\n", black!()).as_str());
        result.push_str(reset!());
        print!("{}", result);
    }
}

pub struct Reports {
    errors: Vec<CompileError>,
}

impl Reports {
    pub fn new() -> Reports {
        Reports { errors: Vec::new() }
    }

    pub fn add_error(&mut self, error: CompileError) {
        self.errors.push(error);
    }

    pub fn has_errors(&self) -> bool {
        self.errors.len() > 0
    }

    pub fn handle_errors(&self) -> bool {
        let mut should_exit = false;
        for error in &self.errors {
            error.print();
            if let ErrorType::Error = error.error_type {
                should_exit = true;
            }
        }
        return should_exit;
    }
}
