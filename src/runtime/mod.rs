use crate::ast::node::atom_node::Symbol;
use crate::ast::node::Node;
use crate::value::NValue;
use std::collections::HashMap;
use std::ops::Deref;

mod environment;

#[derive(Debug)]
pub struct Runtime {
    bindings: HashMap<Symbol, NValue>,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            bindings: HashMap::new(),
        }
    }

    pub fn resolve(&self, symbol: &Symbol) -> NValue {
        self.bindings
            .get(symbol)
            .map(|value| value.clone())
            .unwrap_or(NValue::Nil)
    }

    pub fn define(&mut self, symbol: Symbol, value: NValue) {
        self.bindings.insert(symbol, value);
    }

    pub fn execute(&mut self, node: &Node) -> NValue {
        match node {
            Node::Atom(node) => node.execute(self),
            Node::Definition(node) => node.execute(self),
            Node::List(node) => node.execute(self),
            Node::If(node) => node.execute(self),
            Node::Function(node) => node.execute(self),
            Node::FunctionCall(node) => node.execute(self),
            Node::While(node) => node.execute(self),
        }
    }
}

pub trait Execution {
    fn execute(&self, runtime: &mut Runtime) -> NValue;
}
