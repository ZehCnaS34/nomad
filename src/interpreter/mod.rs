mod context;
use crate::result::RuntimeResult;
use context::Context;
mod value;
use value::{Symbol, Value};
use execution::Execute;
use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use std::sync::Mutex;

use crate::ast::{node, parser::AST, Tag};

pub(crate) trait Introspection {
    fn is_truthy(&self) -> bool;

    fn show(self, label: &str) -> Self;
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
        let mut context = Context::new();
        context.new_namespace(Symbol::from("nomad.core"));
        let mut queue = VecDeque::new();
        queue.push_back(ast.root.unwrap());
        Interpreter {
            ast,
            context,
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
        let node = self.ast.get(&tag).unwrap();
        node.execute(&self)
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
        println!("{:?} {:?}", symbol, value);
        self.context.define(symbol, value);
        println!("{:#?}", self.context);
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

mod execution {
    use crate::ast::tag::*;
    use crate::ast::node::*;
    use super::Interpreter;
    use super::value::*;

    pub trait Execute {
        fn execute(&self, interpreter: &Interpreter) -> Value;
    }

    impl Execute for Node {
        fn execute(&self, interpreter: &Interpreter) -> Value {
            match self {
                Node::Boolean(node) => Value::Boolean(node.value()),
                Node::Definition(node) => node.execute(interpreter),
                Node::Do(node) => node.execute(interpreter),
                Node::Function(node) => todo!(),
                Node::FunctionCall(node) => todo!(),
                Node::If(node) => node.execute(interpreter),
                Node::Keyword(..) => todo!("keywords are no done"),
                Node::List(node) => todo!(),
                Node::Loop(node) => todo!(),
                Node::Nil => Value::Nil,
                Node::Number(number) => Value::Number(number.value()),
                Node::Program(node) => node.execute(interpreter),
                Node::Recur(node) => todo!(),
                Node::String(node) => Value::String(String::from(node.value())),
                Node::Symbol(node) => Value::Symbol(Symbol::from_node(node.clone())),
                Node::Vector(node) => todo!(),
                Node::While(node) => node.execute(interpreter),
            
            }
        }
    }

    impl Execute for ProgramNode {
        fn execute(&self, interpreter: &Interpreter) -> Value {
            let mut result = Value::Nil;
            for tag in self.expressions() {
                result = interpreter.interpret_tag(tag);
            }
            result
        }
    }

    impl Execute for IfNode {
        fn execute(&self, interpreter: &Interpreter) -> Value {
            if interpreter.interpret_tag(self.condition).is_truthy() {
                interpreter.interpret_tag(self.true_branch)
            } else {
                interpreter.interpret_tag(self.false_branch)
            }
        }
    }

    impl Execute for WhileNode {
        fn execute(&self, interpreter: &Interpreter) -> Value {
            loop {
                let condition = interpreter.interpret_tag(self.condition());
                if !condition.is_truthy() {
                    break;
                }
                for tag in self.body() {
                    interpreter.interpret_tag(tag);
                }
            }
            Value::Nil
        }
    }

    impl Execute for DefinitionNode {
        fn execute(&self, interpreter: &Interpreter) -> Value {
            let ident = interpreter.interpret_tag(self.ident());
            if !ident.is_valid_identifier() {
                panic!("invalid identifier")
            }
            let ident = ident.take_symbol().expect("Ident should be a symbol");
            let value = interpreter.interpret_tag(self.value());
            interpreter.define(ident, value)
        }
    }

    impl Execute for DoNode {
        fn execute(&self, interpreter: &Interpreter) -> Value {
            let mut result = Value::Nil;
            for tag in self.expressions() {
                result = interpreter.interpret_tag(tag);
            }
            result
        }
    }
}
