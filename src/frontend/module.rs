use std::path::Path;
use crate::ast::nodes::AST;
use crate::ast::nodes::Node;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamespacePath {
    segments: Vec<String>
}

impl NamespacePath {
    pub fn new(segments: Vec<String>) -> NamespacePath {
        NamespacePath {
            segments
        }
    }

    pub fn display(&self) -> String {
        let mut output = String::new();
        for segment in &*self.segments {
            if output.len() != 0 { output.push_str("::"); }
            output.push_str(segment.as_str());
        }
        output
    }

    pub fn get_segments(&self) -> &Vec<String> {
        &self.segments
    }

    pub fn from_path(path: String) -> Self {
        Self::new(Path::new(&path).with_extension("").as_path().iter().map(|x|x.to_str().unwrap().to_string()).collect())
    }

    pub fn push(&mut self, segment: String) {
        self.segments.push(segment);
    }

    pub fn push_path(&mut self, path: NamespacePath) {
        for segment in path.get_segments() {
            self.push(segment.clone());
        }
    }
}

impl ToString for NamespacePath {
    fn to_string(&self) -> String {
        self.display()
    }
}


#[derive(Debug, Clone)]
pub struct Module<T: Clone + std::fmt::Debug = Node> {
    path: NamespacePath,
    file_name: Option<String>,
    top: Option<AST<T>>,
}

impl<T: Clone + std::fmt::Debug> Module<T> {
    pub fn new(path: NamespacePath, file_name: Option<String>) -> Self {
        Module::<T> {
            path,
            file_name,
            top: None,
        }
    }

    pub fn set_top(&mut self, top: AST<T>) {
        self.top = Some(top);
    }

    pub fn get_top(&self) -> &AST<T> {
        &(self.top.as_ref().unwrap())
    }

    pub fn get_top_mut(&mut self) -> &mut AST<T> {
        self.top.as_mut().unwrap()
    }

    pub fn get_path(&self) -> &NamespacePath {
        &self.path
    }

    pub fn get_file_name(&self) -> &Option<String> {
        &self.file_name
    }
}
