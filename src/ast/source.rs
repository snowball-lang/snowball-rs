#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub path: String,
    pub line: usize,
    pub column: usize,
    pub width: usize,
}

impl SourceLocation {
    pub fn new(path: String, line: usize, column: usize, width: usize) -> SourceLocation {
        SourceLocation {
            path,
            line,
            column,
            width,
        }
    }

    pub fn with_width(&self, width: usize) -> SourceLocation {
        SourceLocation {
            path: self.path.clone(),
            line: self.line,
            column: self.column,
            width,
        }
    }

    pub fn dummy() -> SourceLocation {
        SourceLocation {
            path: String::new(),
            line: 0,
            column: 0,
            width: 0,
        }
    }
}

struct PathHolder {
    path: String,
}

impl PathHolder {
    pub fn new(path: String) -> PathHolder {
        PathHolder { path }
    }
}
