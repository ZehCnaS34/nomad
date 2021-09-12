#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    Symbol(Symbol),
}

impl Value {
    pub fn as_symbol(&self) -> Option<&Symbol> {
        match self {
            Value::Symbol(symbol) => Some(symbol),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Symbol {
    name: String,
    namespace: Option<String>,
}

impl Symbol {
    pub fn name(&self) -> &str {
        &self.name[..]
    }
    pub fn is_qualified(&self) -> bool {
        self.namespace.is_some()
    }
    pub fn from(value: &str) -> Symbol {
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
