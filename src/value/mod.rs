use crate::ast::node::atom_node::Symbol;
use crate::ast::node::Node;
use std::fmt;
use std::ops::Deref;
use std::fmt::Formatter;

mod string;

#[derive(Debug, Clone)]
pub enum NValue {
    Nil,
    NString(String),
    NVar,
    NBoolean(bool),
    NNumber(Box<f64>),
    NFunction(Function),
}

impl fmt::Display for NValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use NValue::*;
        match self {
            Nil => write!(f, "nil"),
            NString(value) => write!(f, "{}", value),
            NVar => write!(f, "var"),
            NBoolean(value) => write!(f, "{}", value),
            NNumber(value) => write!(f, "{}", &*value),
            NFunction(value) => write!(f, "[function]"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Function {
    Multiplication,
    Addition,
    Subtraction,
    Division,
    LessThan,
    GreaterThan,
    Println,
    Runtime {
        parameters: Vec<Symbol>,
        body: Vec<Node>,
    },
}

impl NValue {
    pub fn is_function(&self) -> bool {
        match self {
            NValue::NFunction(_) => true,
            _ => false,
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            NValue::NBoolean(value) => *value,
            _ => false,
        }
    }

    pub fn add(lhs: &NValue, rhs: &NValue) -> NValue {
        use NValue::*;
        match (lhs, rhs) {
            (NNumber(lhs), NNumber(rhs)) => NNumber(Box::new(lhs.clone().deref() + rhs.clone().deref())),
            (NString(lhs), NString(rhs)) => NString(format!("{}{}", &lhs[..], &rhs[..])),
            _ => Nil,
        }
    }

    pub fn multiply(lhs: &NValue, rhs: &NValue) -> NValue {
        use NValue::*;
        match (lhs, rhs) {
            (NNumber(lhs), NNumber(rhs)) => NNumber(Box::new(lhs.clone().deref() * rhs.clone().deref())),
            _ => Nil,
        }
    }

    pub fn divide(lhs: &NValue, rhs: &NValue) -> NValue {
        use NValue::*;
        match (lhs, rhs) {
            (NNumber(lhs), NNumber(rhs)) => NNumber(Box::new(lhs.clone().deref() / rhs.clone().deref())),
            _ => Nil,
        }
    }

    pub fn subtract(lhs: &NValue, rhs: &NValue) -> NValue {
        use NValue::*;
        match (lhs, rhs) {
            (NNumber(lhs), NNumber(rhs)) => NNumber(Box::new(lhs.clone().deref() - rhs.clone().deref())),
            _ => Nil,
        }
    }

    pub fn less_than(lhs: &NValue, rhs: &NValue) -> NValue {
        use NValue::*;
        match (lhs, rhs) {
            (NNumber(lhs), NNumber(rhs)) => NBoolean(lhs.clone().deref() < rhs.clone().deref()),
            _ => Nil,
        }
    }

    pub fn greater_than(lhs: &NValue, rhs: &NValue) -> NValue {
        use NValue::*;
        match (lhs, rhs) {
            (NNumber(lhs), NNumber(rhs)) => NBoolean(lhs.clone().deref() > rhs.clone().deref()),
            _ => Nil,
        }
    }
}
