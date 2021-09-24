use std::fmt;

#[derive(Debug, Clone)]
pub struct Var {
    name: String,
    namespace: String,
}

impl Var {
    pub fn name(&self) -> &str {
        &self.name[..]
    }

    pub fn namespace(&self) -> &str {
        &self.namespace[..]
    }
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#'{}/{}", self.namespace(), self.name())
    }
}
