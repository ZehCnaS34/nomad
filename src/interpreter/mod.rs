use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

use crate::ast::node::Node;
use crate::ast::node::SymbolNode;
use crate::ast::node::VectorNode;
use crate::ast::parser::AST;
use crate::ast::Tag;
use crate::prelude::*;
use crate::result::runtime::ErrorKind;
use crate::result::Result;

mod context;
mod execution;
mod frame;
mod operation;
mod value;

use context::Context;
use context::Dump;
use execution::Execute;

pub use operation::Compare;
pub use operation::Concat;
pub use operation::Introspection;
pub use operation::Length;
pub use operation::Math;

use value::nf;
use value::Symbol;
pub use value::Value;

pub trait NodeQuery {
    fn get_nodes(&self, tags: Vec<Tag>) -> Result<Vec<Node>>;
    fn get_node(&self, tag: Tag) -> Result<Node>;
    fn get_vector(&self, tag: Tag) -> Result<VectorNode>;
    fn get_symbol(&self, tags: Tag) -> Result<SymbolNode>;
    fn get_symbols(&self, tags: &Vec<Tag>) -> Result<Vec<SymbolNode>>;
}

impl NodeQuery for Interpreter {
    fn get_nodes(&self, tags: Vec<Tag>) -> Result<Vec<Node>> {
        let mut nodes = vec![];
        for tag in tags {
            nodes.push(self.get_node(tag)?);
        }
        Ok(nodes)
    }

    fn get_node(&self, tag: Tag) -> Result<Node> {
        self.ast
            .as_ref()
            .and_then(|ast| ast.get(&tag).cloned())
            .ok_or(ErrorKind::NodeNotFound)
    }

    fn get_vector(&self, tag: Tag) -> Result<VectorNode> {
        if let Node::Vector(node) = self.get_node(tag)? {
            Ok(node)
        } else {
            Err(ErrorKind::MissingNode)
        }
    }

    fn get_symbol(&self, tag: Tag) -> Result<SymbolNode> {
        if let Node::Symbol(node) = self.get_node(tag)? {
            Ok(node)
        } else {
            Err(ErrorKind::MissingNode)
        }
    }

    fn get_symbols(&self, tags: &Vec<Tag>) -> Result<Vec<SymbolNode>> {
        let mut symbols = vec![];
        for tag in tags {
            symbols.push(self.get_symbol(*tag)?);
        }
        Ok(symbols)
    }
}

#[derive(Debug)]
pub struct Interpreter {
    ast: Option<AST>,
    context: Context,
    values: Mutex<HashMap<value::Symbol, value::Value>>,
}

impl Interpreter {
    pub fn boot() -> Result<Interpreter> {
        let context = {
            use nf::*;
            let context = Context::new();
            context.new_namespace(Symbol::from("nomad.core"))?;
            context.define("now", Now)?;
            context.define("=", Equal)?;
            context.define("mod", Modulus)?;
            context.define("<", LessThan)?;
            context.define(">", GreaterThan)?;
            context.define("+", Plus)?;
            context.define("*", Multiply)?;
            context.define("/", Divide)?;
            context.define("-", Minus)?;
            context.define("get", Get)?;
            context.define("conj", Conj)?;
            context.define("count", Count)?;
            context.define("print", Print)?;
            context.define("println", Println)?;
            context.define("*version*", Value::make_number(0.into()))?;
            context
        };
        Ok(Interpreter {
            ast: None,
            context,
            values: Mutex::new(HashMap::new()),
        })
    }

    pub fn dump_context(&self) {
        self.context.dump();
    }

    pub(crate) fn put(&self, symbol: value::Symbol, atom: value::Value) {
        let mut values = self.values.lock().unwrap();
        values.insert(symbol, atom);
    }

    pub fn interpret_tag(&self, tag: Tag) -> Result<Value> {
        let node = self.get_node(tag)?;
        node.execute(&self)
    }

    pub fn resolve(&self, symbol: &Symbol) -> Result<Value> {
        if let Ok(value) = self.context.get(symbol) {
            Ok(value)
        } else {
            self.context.resolve(symbol)
        }
    }

    pub fn interpret_and_resolve_tags(&self, tags: Vec<Tag>) -> Result<Vec<Value>> {
        let mut values = vec![];
        for tag in tags {
            values.push(self.interpret_and_resolve_tag(tag)?);
        }
        Ok(values)
    }

    pub fn interpret_and_resolve_tag(&self, tag: Tag) -> Result<Value> {
        let value = self.interpret_tag(tag)?;
        if let Some(symbol) = value.as_symbol() {
            println!("{:?}", symbol);
            self.resolve(symbol)
        } else {
            Ok(value)
        }
    }

    pub fn root(&self) -> Option<Tag> {
        self.ast
            .as_ref()
            .and_then(|ast| ast.root)
            .map(|tag| tag.clone())
    }

    pub fn eval(&mut self, ast: AST) -> Result<Value> {
        self.ast = Some(ast);
        if let Some(root) = self.root() {
            self.interpret_and_resolve_tag(root)
        } else {
            Ok(Value::Nil)
        }
    }

    pub fn define(&self, symbol: Symbol, value: Value) -> Result<Value> {
        self.context.define(symbol.clone(), value).map(Value::Var)
    }

    pub fn set(&self, symbol: Symbol, value: Value) {
        self.context.set(symbol, value);
    }
}
