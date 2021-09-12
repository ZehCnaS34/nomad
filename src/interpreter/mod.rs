mod context;
use crate::result::RuntimeResult;
use context::Context;
mod value;
use value::{Symbol, Value};

use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use std::sync::Mutex;

use crate::ast::{node, parser::AST, Tag};

pub(crate) trait Introspection {
    fn is_truthy(&self) -> bool;

    fn show(self, label: &str) -> Self;
}

pub trait Execute {
    fn execute(&self, interpreter: &Interpreter, own_tag: Tag) -> value::Value;
}

pub trait Operation {
    type Val;
    type Err;
    fn add(&self, rhs: &Self) -> Result<Self::Val, Self::Err>;
    fn sub(&self, rhs: &Self) -> Result<Self::Val, Self::Err>;
    fn mul(&self, rhs: &Self) -> Result<Self::Val, Self::Err>;
    fn div(&self, rhs: &Self) -> Result<Self::Val, Self::Err>;
    fn eq(&self, rhs: &Self) -> Result<Self::Val, Self::Err>;
    fn lt(&self, rhs: &Self) -> Result<Self::Val, Self::Err>;
    fn gt(&self, rhs: &Self) -> Result<Self::Val, Self::Err>;
    fn imod(&self, rhs: &Self) -> Result<Self::Val, Self::Err>;
}

#[derive(Debug)]
pub struct Interpreter {
    ast: AST,
    context: Context,
    values: Mutex<HashMap<value::Symbol, value::Value>>,
}

impl Interpreter {
    fn new(ast: AST) -> Interpreter {
        let mut queue = VecDeque::new();
        queue.push_back(ast.root.unwrap());
        Interpreter {
            ast,
            context: Context::new(),
            values: Mutex::new(HashMap::new()),
        }
    }

    #[inline]
    pub(crate) fn put(&self, symbol: value::Symbol, atom: value::Value) {
        let mut values = self.values.lock().unwrap();
        values.insert(symbol, atom);
    }

    #[inline]
    pub(crate) fn interpret_tag(&self, tag: Tag) -> Value {
        todo!();
        // // println!("tag {:?}", tag);
        // let node = self.ast.get(&tag).unwrap();
        // let atom = node.execute(&self, tag);
        // self.resolve(atom).show("label")
    }

    pub fn resolve(&self, atom: Value) -> RuntimeResult<Value> {
        let symbol = atom
            .as_symbol()
            .ok_or(crate::result::runtime::ErrorKind::NotDefined)?;
        let mut values = self.values.lock().expect("Failed to lock values");
        let value = values
            .get(&symbol)
            .ok_or(crate::result::runtime::ErrorKind::NotDefined)?;
        Ok(value.clone())
    }

    pub fn define(&self, symbol: Symbol, value: Value) -> Value {
        todo!("define needs to be implemented");
        // let mut values = self.values.lock().expect("Failed to lock values");
        // values.insert(symbol.clone(), value);
        // ::Var(Var::make(("awesome", symbol.name())))
    }

    fn run(&self) {
        if let Some(tag) = self.ast.root {
            println!("result = {:?}", self.interpret_tag(tag));
        }
    }
}

pub fn interpret(ast: AST) {
    let env = Interpreter::new(ast);
    env.run();
}
