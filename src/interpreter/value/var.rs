use std::fmt;

#[derive(Debug, Clone)]
pub struct Var {
    pub name: String,
    pub namespace: String,
}

impl Var {}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#'{}/{}", self.namespace, self.name)
    }
}
