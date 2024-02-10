
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
}

struct PathHolder {
    path: String,
}

impl PathHolder {
    pub fn new(path: String) -> PathHolder {
        PathHolder { path }
    }
}
