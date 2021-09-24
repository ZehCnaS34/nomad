use std::fmt;
use crate::ast::node::SymbolNode;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Symbol {
    name: String,
    namespace: Option<String>,
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

impl From<&str> for Symbol {
    fn from(value: &str) -> Self {
        if value.len() == 1 {
            Symbol {
                name: String::from(value),
                namespace: None,
            }
        } else if let Some(index) = value.find('/') {
            Symbol {
                name: String::from(&value[index + 1..]),
                namespace: Some(String::from(&value[..index])),
            }
        } else {
            Symbol {
                name: String::from(value),
                namespace: None,
            }
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

impl From<(&'static str, &'static str)> for Symbol{
    fn from((ns, n): (&'static str, &'static str)) -> Self {
        Symbol {
            name: n.into(),
            namespace: Some(ns.into()),
        }
    }
}

impl From<(&'static str)> for Symbol {
    fn from((n): &'static str) -> Self {
        Symbol {
            name: n.into(),
            namespace: None,
        }
    }
}
