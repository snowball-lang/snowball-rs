use std::{collections::HashMap, path::Path};

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
}

impl ToString for NamespacePath {
    fn to_string(&self) -> String {
        self.display()
    }
}


#[derive(Debug, Clone)]
pub struct Module<T: Clone> {
    path: NamespacePath,
    file_name: Option<String>,
    top: Vec<T>,
}

impl<T: Clone> Module<T> {
    pub fn new(path: NamespacePath, file_name: Option<String>) -> Self {
        Module {
            path,
            file_name,
            top: Vec::new(),
        }
    }

    pub fn set_top(&mut self, top: Vec<T>) {
        self.top = top;
    }
}
