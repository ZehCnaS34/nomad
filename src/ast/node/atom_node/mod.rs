use crate::runtime::{Execution, Runtime};
use crate::value::NValue;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum AtomNode {
    Nil,
    Rational(f64),
    Integer(i32),
    Boolean(bool),
    Symbol(Symbol),
    String(String),
    Var(Var),
}

impl fmt::Display for AtomNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AtomNode::Nil => write!(f, "nil"),
            AtomNode::Rational(n) => write!(f, "{}", n),
            AtomNode::Integer(i) => write!(f, "{}", i),
            AtomNode::Boolean(b) => write!(f, "{}", b),
            AtomNode::Symbol(s) => write!(f, "{}", s),
            AtomNode::String(s) => write!(f, "{:?}", s),
            AtomNode::Var(s) => write!(f, "{:?}", s),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct Var {
    literal: String,
}

pub enum ParseError {
    Eof,
}

impl FromStr for AtomNode {
    type Err = ParseError;

    fn from_str(atom: &str) -> Result<Self, Self::Err> {
        if atom == "nil" {
            Ok(AtomNode::Nil)
        } else if atom == "true" {
            Ok(AtomNode::Boolean(true))
        } else if atom == "false" {
            Ok(AtomNode::Boolean(false))
        } else if atom.starts_with('"') {
            // TODO: Handle string escaping
            Ok(AtomNode::String(String::from(&atom[1..atom.len() - 1])))
        } else if let Ok(value) = atom.parse() {
            Ok(AtomNode::Rational(value))
        } else {
            Ok(AtomNode::Symbol(atom.into()))
        }
    }
}

impl Execution for AtomNode {
    fn execute(&self, runtime: &mut Runtime) -> NValue {
        match self {
            AtomNode::Symbol(symbol) => runtime.resolve(symbol),
            AtomNode::Rational(number) => NValue::NNumber(Box::new(*number)),
            AtomNode::Integer(number) => NValue::NNumber(Box::new((*number) as f64)),
            AtomNode::String(string) => NValue::NString(string.clone()),
            AtomNode::Nil => NValue::Nil,
            AtomNode::Boolean(bool) => NValue::NBoolean(*bool),
            _ => NValue::Nil,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct Symbol {
    literal: String,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.literal)
    }
}

impl From<&str> for Symbol {
    fn from(value: &str) -> Symbol {
        Symbol {
            literal: String::from(value),
        }
    }
}

impl Symbol {
    pub fn name(&self) -> &str {
        if self.literal.len() <= 1 {
            &self.literal[..]
        } else if let Some(index) = &self.literal[..].find('/') {
            &self.literal[index + 1..]
        } else {
            &self.literal[..]
        }
    }

    pub fn namespace(&self) -> Option<&str> {
        if self.literal.len() <= 1 {
            None
        } else if let Some(index) = &self.literal[..].find('/') {
            Some(&self.literal[0..index + 0])
        } else {
            None
        }
    }

    pub fn get_namespace(&self) -> Option<Symbol> {
        self.namespace().map(|namespace| namespace.into())
    }

    pub fn is_qualified(&self) -> bool {
        self.literal.len() > 1 && self.literal[..].contains('/')
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn properly_inspect_symbol_name() {
        let sym: Symbol = "awesome".into();
        assert_eq!(sym.name(), "awesome");
        let sym: Symbol = "nomad.core/add".into();
        assert_eq!(sym.name(), "add");
    }

    #[test]
    fn properly_inspect_symbol_namespace() {
        let sym: Symbol = "nomad.core/add".into();
        assert_eq!(sym.namespace(), Some("nomad.core"));
    }

    #[test]
    fn inspect_divide_symbol() {
        let sym: Symbol = "/".into();
        assert_eq!(sym.name(), "/");
    }

    #[test]
    fn meta_information() {
        let sym: Symbol = "nomad.core/information".into();
        assert_eq!(sym.is_qualified(), true);
    }

    #[test]
    fn create_namespace_symbol_from_symbol() {
        let full_sym: Symbol = "nomad.core/information".into();
        let ns_sym = full_sym.get_namespace().unwrap();
        assert_eq!(ns_sym.name(), "nomad.core");
    }
}
