use crate::interpreter::operation::Conj;
use crate::interpreter::operation::Introspection;
use crate::interpreter::operation::Length;
use crate::interpreter::operation::Lookup;
use crate::result::runtime::ErrorKind as Error;
use crate::result::RuntimeResult as Result;

mod boolean;
mod function;
mod hash_map;
mod keyword;
mod list;
mod number;
mod string;
mod symbol;
mod var;
mod vector;

use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

pub use boolean::Boolean;
pub use function::Function;
pub use function::NativeFunction;
pub use number::Number;
pub use string::String;
pub use symbol::Symbol;
pub use var::Var;
pub use vector::Vector;

pub const NIL: Value = Value::Nil;

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Boolean(Boolean),
    Number(Number),
    String(String),
    Symbol(Symbol),
    Var(Var),
    Function(Function),
    NativeFunction(NativeFunction),
    Vector(Vector<Value>),
}

impl Lookup for Value {
    type Item = Value;
    type Key = Value;
    type Err = Error;

    fn lookup(&self, key: Self::Key) -> Result<&Self::Item> {
        match (self, key) {
            (Value::Vector(vector), Value::Number(number)) => vector.lookup(number),
            (left, right) => {
                println!("left {} {}", left, right);
                Err(Error::General("un supported lookup"))
            }
        }
    }
}

impl Conj for Value {
    type Container = Value;
    type Value = Value;
    type Err = Error;

    fn conj(&self, value: Self::Value) -> Result<Self::Container> {
        match self {
            Value::Vector(vector) => Ok(Value::Vector(vector.push(value))),
            value => Err(Error::General("Unsupported operation.")),
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value.into())
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Number(value.into())
    }
}

impl From<NativeFunction> for Value {
    fn from(f: NativeFunction) -> Self {
        Value::NativeFunction(f)
    }
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
            Value::Function(value) => {
                if let Some(name) = &value.name {
                    write!(f, "[fn:{}]", name)
                } else {
                    write!(f, "[fn:anonymous]")
                }
            }
            Value::NativeFunction(value) => write!(f, "[native]"),
            Value::Vector(vector) => write!(f, "{}", vector),
        }
    }
}

impl Value {
    pub fn make_number(value: f64) -> Value {
        Value::Number(Number { value })
    }

    pub fn make_string(value: &str) -> Value {
        Value::String(String {
            value: value.to_string(),
        })
    }

    pub fn make_nil() -> Value {
        Value::Nil
    }

    pub fn make_bool(value: bool) -> Value {
        Value::Boolean(Boolean { value })
    }

    pub fn show(self) -> Self {
        println!("value {}", self);
        return self;
    }

    pub fn is_local_identifier(&self) -> bool {
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
}
