
#[derive(Debug, Clone, PartialEq)]
pub enum ExternalLinkage {
    C,
    Snowball,
    System
}

#[derive(Debug, Clone, PartialEq)]
pub enum Linkage {
    Internal,
    External
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstAttrs {
    Invalid,
    Privacy(bool),
    Static,
    Mut,
    Cfg(String),
    Test,
    Benchmark,
    Override,
    Abstract,
    Final,
    Inline,
    Const,
    External(ExternalLinkage),
    BuiltIn,
    Export,
    Unsafe,
    NoMangle,
    NoInline,
    NoOptimize,
    LLVMFunct,
    NotImplemented,
    Deprecated(String),
    Unstable(String),
    Linkage(Linkage),
    NoConstructors
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttrHandler {
    attrs: Vec<AstAttrs>
}

impl AttrHandler {
    pub fn new() -> Self {
        AttrHandler { attrs: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.attrs.clear();
    }

    pub fn add_attr(&mut self, attr: AstAttrs) {
        if !self.attrs.contains(&attr) {
            self.attrs.push(attr);
        }
    }

    pub fn get_attrs(&self) -> &Vec<AstAttrs> {
        &self.attrs
    }

    pub fn is_pub(&self) -> bool {
        for attr in &self.attrs {
            match attr {
                AstAttrs::Privacy(true) => return true,
                _ => {}
            }
        }
        false
    }

    pub fn is_priv(&self) -> bool {
        !self.is_pub()
    }
}
