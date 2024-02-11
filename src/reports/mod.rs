use crate::{black, blue, bold, compiler::file_loader, red, reset, yellow};
use std::io::Write;

pub enum Error {
    UnexpectedChar(char),
    UnknownEscapeSequence(char),
    UnexpectedToken(String),
    ExpectedItem(String, String),
    UnexpectedEOF,
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::UnexpectedChar(c) => format!("unexpected character: '{}'", c),
            Error::UnexpectedEOF => "unexpected end of file".to_string(),
            Error::UnknownEscapeSequence(c) => format!("unknown escape sequence: '\\{}'", c),
            Error::UnexpectedToken(t) => format!("unexpected token: '{}'", t.replace("\n", "\\n")),
            Error::ExpectedItem(item, after) => format!("expected '{}' after '{}'!", item, after),
        }
    }
}

pub enum ErrorType {
    Error,
    Warning,
}

#[derive(Clone, Default)]
pub struct ErrorInfo {
    pub help: Option<String>,
    pub note: Option<String>,
    pub info: Option<String>,
    pub see: Option<String>,
}

pub struct CompileError {
    error_type: ErrorType,
    message: Error,
    location: crate::ast::source::SourceLocation,

    info: ErrorInfo,
}

impl CompileError {
    pub fn new(error_type: Error, location: crate::ast::source::SourceLocation) -> CompileError {
        CompileError {
            error_type: ErrorType::Error,
            message: error_type,
            location,
            info: ErrorInfo {
                help: None,
                note: None,
                info: None,
                see: None,
            },
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
            info: ErrorInfo {
                help: None,
                note: None,
                info: None,
                see: None,
            },
        }
    }

    pub fn with_info(mut self, info: ErrorInfo) -> CompileError {
        self.info = info;
        self
    }

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
            "\n{}{}{}: {}{}\n at [{}{}{}:{}{}{}:{}{}{}]\n",
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
            bold!()
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
                        if i == (self.location.column) {
                            result_line.push_str(reset!());
                            result_line.push_str(bold!());
                        }
                        result_line.push(c);
                        if i == (self.location.column + self.location.width - 1) {
                            result_line.push_str(reset!());
                            result_line.push_str(black!());
                        }
                    }
                    result.push_str(format!("{}{}{:3} | {}{}{}\n", black!(), bold!(), line, reset!(), black!(), result_line).as_str());
                    result.push_str(format!("{}{}    | ", bold!(), black!()).as_str());
                    for i in 0..l.len() {
                        if i == self.location.column {
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
                    result.push_str(&format!("{}{}{:3} | {}{}{}\n", bold!(), black!(), line, reset!(), black!(), l));
                }
            }
        }
        let mut append_extra = false;
        result.push_str(format!("{}{}    |\n", black!(), bold!()).as_str());
        if let Some(help) = &self.info.help {
            result.push_str(format!("{}{}help: {}{}\n", reset!(), bold!(), reset!(), help).as_str());
            append_extra = true;
        }
        if let Some(note) = &self.info.note {
            result.push_str(format!("{}{}note: {}{}\n", reset!(), bold!(), reset!(), note).as_str());
            append_extra = true;
        }
        if let Some(info) = &self.info.info {
            result.push_str(format!("{}{}info: {}{}\n", reset!(), bold!(), reset!(), info).as_str());
            append_extra = true;
        }
        if let Some(see) = &self.info.see {
            result.push_str(format!("{}{} see: {}{}\n", reset!(), bold!(), reset!(), see).as_str());
            append_extra = true;
        }
        if append_extra {
            result.push_str(format!("{}{}    |\n", black!(), bold!()).as_str());
        }
        result.push_str(reset!());
        print!("{}", result);
    }

    fn get_help_msg(&self, msg: String) -> String {
        let mut result = String::new();
        for (i, line) in msg.lines().enumerate() {
            if i == 0 {
                result.push_str(line);
            } else {
                result.push_str(format!("\n{}    | {}{}", black!(), reset!(), line).as_str());
            }
        }
        result
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
