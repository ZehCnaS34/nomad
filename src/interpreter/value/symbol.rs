use crate::ast::node::SymbolNode;
use std::fmt;

type STR = &'static str;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Symbol {
    pub name: String,
    pub namespace: Option<String>,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(namespace) = self.namespace() {
            write!(f, "{}/{}", namespace, self.name())
        } else {
            write!(f, "{}", self.name())
        }
    }
}

impl Symbol {
    pub fn qualify(&mut self, name: &str) {
        self.namespace = Some(String::from(name))
    }

    pub fn from_node(node: SymbolNode) -> Symbol {
        Symbol {
            name: node.name().to_string(),
            namespace: node.namespace().map(|namespace| namespace.to_string()),
        }
    }

    pub fn name(&self) -> &str {
        &self.name[..]
    }

    pub fn namespace(&self) -> Option<&str> {
        self.namespace.as_ref().map(|namespace| &namespace[..])
    }

    pub fn is_qualified(&self) -> bool {
        self.namespace.is_some()
    }
}

impl From<(STR, STR)> for Symbol {
    fn from((ns, n): (STR, STR)) -> Self {
        Symbol {
            name: n.into(),
            namespace: Some(ns.into()),
        }
    }
}

impl From<(STR)> for Symbol {
    fn from((n): (STR)) -> Self {
        Symbol {
            name: n.into(),
            namespace: None,
        }
    }
}
