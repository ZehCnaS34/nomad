use crate::ast::node::SymbolNode;
use crate::ast::Tag;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    Symbol(Symbol),
    Var(Var),
    Function(Function),
    NativeFunction(NativeFunction),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Boolean(value) => write!(f, "{}", value),
            Value::Number(value) => write!(f, "{}", value),
            Value::String(value) => write!(f, "{}", value),
            Value::Symbol(value) => write!(f, "{}", value),
            Value::Var(value) => write!(f, "{}", value),
            Value::Function(value) => write!(f, "[fn]"),
            Value::NativeFunction(value) => write!(f, "[native]"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum NativeFunction {
    Plus,
    Minus,
    Print,
    Println,
    LessThan,
    GreaterThan,
}

impl Value {
    pub fn show(self) -> Self {
        println!("value {}", self);
        return self;
    }

    pub fn is_valid_identifier(&self) -> bool {
        self.as_symbol()
            .map(|symbol| !symbol.is_qualified())
            .unwrap_or(false)
    }

    pub fn as_symbol(&self) -> Option<&Symbol> {
        match self {
            Value::Symbol(symbol) => Some(symbol),
            _ => None,
        }
    }

    pub fn take_function(self) -> Option<Function> {
        match self {
            Value::Function(function) => Some(function),
            _ => None,
        }
    }

    pub fn take_symbol(self) -> Option<Symbol> {
        match self {
            Value::Symbol(symbol) => Some(symbol),
            _ => None,
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(v) => *v,
            Value::Nil => false,
            _ => true,
        }
    }
}

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

    pub fn to_var(&self) -> Var {
        let name = self.name();
        let namespace = self.namespace().expect("Var's must be qualified");
        Var {
            name: String::from(name),
            namespace: String::from(namespace),
        }
    }
}

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

#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Tag,
    pub body: Vec<Tag>,
}
