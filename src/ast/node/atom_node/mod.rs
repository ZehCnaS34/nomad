pub mod symbol;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
pub use symbol::Symbol;

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
