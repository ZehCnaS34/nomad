use super::symbol::Symbol;
use crate::ast::Tag;
use crate::defnode;
use crate::interpreter::Interpreter;
use crate::prelude::*;

pub enum Arity {
    Fixed(usize),
    Or(usize, usize),
    Any,
    MinOne,
    None,
}

pub trait Function: fmt::Debug {
    fn arity(&self) -> Arity;
    fn name(&self) -> &str;
    fn call(&self, parameters: Vec<Value>, interpreter: &Interpreter) -> Result<Value>;
}

#[derive(Debug, Clone)]
pub struct UserFunction {
    pub name: Option<Symbol>,
    pub parameters: Vec<Symbol>,
    pub body: Vec<Node>,
}

impl Function for UserFunction {
    fn arity(&self) -> Arity {
        Arity::Fixed(self.parameters.len())
    }

    fn name(&self) -> &str {
        // if let Some(name) = &self.name {
        //     &name[..]
        // } else {
        "anonymous"
        // }
    }

    fn call(&self, parameters: Vec<Value>, interpreter: &Interpreter) -> Result<Value> {
        todo!()
    }
}

macro_rules! native_function {
    ($name:ident($params:ident, $interpreter:ident) ($string:literal,$arity:expr) : $body:block) => {
        #[derive(Debug, Clone)]
        pub struct $name;
        impl Function for $name {
            fn arity(&self) -> Arity {
                $arity
            }

            fn name(&self) -> &str {
                $string
            }

            fn call(&self, $params: Vec<Value>, $interpreter: &Interpreter) -> Result<Value> $body
        }
    };
}

native_function! {
    Plus(parameters, int) ("+", Arity::Any) : {
        Err(General("fuck"))
    }
}
native_function! {
    Minus(parameters, int) ("-", Arity::Any) : {
        Err(General("fuck"))
    }
}
native_function! {
    Multiply(parameters, int) ("*", Arity::Any) : {
        Err(General("fuck"))
    }
}
native_function! {
    Divide(parameters, int) ("/", Arity::Any) : {
        Err(General("fuck"))
    }
}
native_function! {
    Modulus(parameters, int) ("mod", Arity::Fixed(2)) : {
        Err(General("fuck"))
    }
}
native_function! {
    Equal(parameters, int) ("=", Arity::Any) : {
        Err(General("fuck"))
    }
}
native_function! {
    LessThan(parameters, int) ("<", Arity::Any) : {
        Err(General("fuck"))
    }
}
native_function! {
    GreaterThan(parameters, int) (">", Arity::Any) : {
        Err(General("fuck"))
    }
}
native_function! {
    Println(parameters, int) ("println", Arity::Any) : {
        Err(General("fuck"))
    }
}
native_function! {
    Print(parameters, int) ("print", Arity::Any) : {
        Err(General("fuck"))
    }
}
native_function! {
    Conj(parameters, int) ("conj", Arity::Fixed(2)) : {
        Err(General("fuck"))
    }
}
native_function! {
    Get(parameters, int) ("get", Arity::Or(2, 3)) : {
        Err(General("fuck"))
    }
}
native_function! {
    Count(parameters, int) ("count", Arity::Fixed(1)) : {
        Err(General("fuck"))
    }
}
native_function! {
    Now(parameters, int) ("now", Arity::None) : {
        Err(General("fuck"))
    }
}

pub mod nf {
    pub use super::{
        Conj, Count, Divide, Equal, Get, GreaterThan, LessThan, Minus, Modulus, Multiply, Now,
        Plus, Print, Println,
    };
}

impl<T> From<T> for Value
    where
        T: Function + 'static,
{
    fn from(f: T) -> Value {
        Value::Function(Arc::new(f))
    }
}
