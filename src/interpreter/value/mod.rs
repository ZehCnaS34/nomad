mod function;
mod number;
mod string;
mod symbol;
mod var;
mod boolean;
mod operation;
use std::fmt;


use function::Function;
use function::NativeFunction;
use number  ::Number;
use string  ::String;
use symbol  ::Symbol;
use var     ::Var;
use boolean ::Boolean;
use crate::interpreter::value::operation::Operation;

pub const NIL: Value = Value::Nil;

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Boolean(Boolean),
    Number(Number),
    String(St),
    Symbol(Symbol),
    Var(Var),
    Function(Function),
    NativeFunction(NativeFunction),
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
            Value::Function(value) => write!(f, "[fn]"),
            Value::NativeFunction(value) => write!(f, "[native]"),
        }
    }
}


impl Value {
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

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(boolean) => boolean.is_truthy(),
            Value::Nil => false,
            _ => true,
        }
    }
}
