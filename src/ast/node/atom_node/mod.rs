mod symbol;
mod var;

use crate::ast::node::atom_node::AtomNode::Boolean;
use crate::ast::node::Tag;
use crate::interpreter::Operation;
use crate::interpreter::{Execute, Interpreter};
use std::cmp::{Eq, Ord, Ordering};
use std::fmt;
use std::ops::Deref;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;
pub use symbol::Symbol;
pub use var::Var;

#[derive(Clone)]
pub enum AtomNode {
    Nil,
    Rational(f64),
    Integer(i32),
    Boolean(bool),
    Symbol(Symbol),
    String(String),
    Vector(Vec<AtomNode>),
    Var(Var),
}

impl fmt::Debug for AtomNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AtomNode::Nil => write!(f, "nil"),
            AtomNode::Rational(n) => write!(f, "{}", n),
            AtomNode::Integer(i) => write!(f, "{}", i),
            AtomNode::Boolean(b) => write!(f, "{}", b),
            AtomNode::Symbol(s) => write!(f, "{}", s),
            AtomNode::String(s) => write!(f, "{:?}", s),
            AtomNode::Var(s) => write!(f, "{:?}", s),
            AtomNode::Vector(s) => write!(f, "{:?}", s),
        }
    }
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
            AtomNode::Vector(s) => write!(f, "{:?}", s),
        }
    }
}

impl AtomNode {
    pub fn as_symbol(&self) -> Option<&Symbol> {
        match self {
            AtomNode::Symbol(symbol) => Some(symbol),
            _ => None,
        }
    }
    pub fn take_symbol(self) -> Option<Symbol> {
        match self {
            AtomNode::Symbol(symbol) => Some(symbol),
            _ => None,
        }
    }

    pub fn is_symbol(&self) -> bool {
        match self {
            AtomNode::Symbol(symbol) => true,
            _ => false,
        }
    }
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

pub enum OperationError {
    CannotAdd,
    CannotDiv,
    CannotMul,
    CannotSub,
    CannotCompare,
}

impl Mul for &AtomNode {
    type Output = Result<AtomNode, OperationError>;
    fn mul(self, rhs: Self) -> Self::Output {
        use AtomNode::{Integer, Rational};
        match (self, rhs) {
            (Rational(l), Rational(r)) => Ok(Rational(l * r)),
            (_, _) => Err(OperationError::CannotMul),
        }
    }
}
impl Div for &AtomNode {
    type Output = Result<AtomNode, OperationError>;
    fn div(self, rhs: Self) -> Self::Output {
        use AtomNode::{Integer, Rational};
        match (self, rhs) {
            (Rational(l), Rational(r)) => Ok(Rational(l / r)),
            (_, _) => Err(OperationError::CannotDiv),
        }
    }
}
impl Sub for &AtomNode {
    type Output = Result<AtomNode, OperationError>;
    fn sub(self, rhs: Self) -> Self::Output {
        use AtomNode::{Integer, Rational};
        match (self, rhs) {
            (Rational(l), Rational(r)) => Ok(Rational(l - r)),
            (_, _) => Err(OperationError::CannotSub),
        }
    }
}
impl Add for &AtomNode {
    type Output = Result<AtomNode, OperationError>;
    fn add(self, rhs: Self) -> Self::Output {
        use AtomNode::{Integer, Rational};
        match (self, rhs) {
            (Rational(l), Rational(r)) => Ok(Rational(l + r)),
            (_, _) => Err(OperationError::CannotAdd),
        }
    }
}

impl PartialEq<Self> for &AtomNode {
    fn eq(&self, other: &Self) -> bool {
        use AtomNode::{Integer, Rational};
        match (self, other) {
            (Rational(l), Rational(r)) => l == r,
            (Integer(l), Rational(r)) => (*l as f64) == *r,
            (Rational(l), Integer(r)) => *l == (*r).into(),
            (_, _) => false,
        }
    }
}

impl PartialOrd<Self> for &AtomNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use AtomNode::{Integer, Rational};
        match (self, other) {
            (Rational(l), Rational(r)) => l.partial_cmp(r),
            (Integer(l), Rational(r)) => (*l as f64).partial_cmp(r),
            (Rational(l), Integer(r)) => (*l).partial_cmp(&(*(r) as f64)),
            (_, _) => None,
        }
    }
}

impl Operation for AtomNode {
    type Val = AtomNode;
    type Err = OperationError;

    fn add(&self, rhs: &Self) -> Result<Self::Val, Self::Err> {
        self + rhs
    }

    fn sub(&self, rhs: &Self) -> Result<Self::Val, Self::Err> {
        self - rhs
    }

    fn mul(&self, rhs: &Self) -> Result<Self::Val, Self::Err> {
        self * rhs
    }

    fn div(&self, rhs: &Self) -> Result<Self::Val, Self::Err> {
        self / rhs
    }

    fn eq(&self, rhs: &Self) -> Result<Self::Val, Self::Err> {
        Ok(AtomNode::Boolean(self == rhs))
    }

    fn lt(&self, rhs: &Self) -> Result<Self::Val, Self::Err> {
        Ok(AtomNode::Boolean(self < rhs))
    }

    fn gt(&self, rhs: &Self) -> Result<Self::Val, Self::Err> {
        Ok(AtomNode::Boolean(self > rhs))
    }
}

impl Execute for AtomNode {
    fn execute(&self, interpreter: &Interpreter, own_tag: Tag) {
        interpreter.set_tag_data(own_tag, self.clone());
    }
}
