use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

use crate::ast::node;
use crate::ast::node::Node;
use crate::ast::node::SymbolNode;
use crate::ast::node::VectorNode;
use crate::ast::parser::AST;
use crate::ast::Tag;
use crate::result::runtime::ErrorKind;
use crate::result::RuntimeResult;

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

use value::Boolean;
use value::Function;
use value::NativeFunction;
use value::Number;
use value::String;
use value::Symbol;
use value::Value;
use value::Var;

pub trait NodeQuery {
    fn get_nodes(&self, tags: Vec<Tag>) -> RuntimeResult<Vec<Node>>;
    fn get_node(&self, tag: Tag) -> RuntimeResult<Node>;
    fn get_vector(&self, tag: Tag) -> RuntimeResult<VectorNode>;
    fn get_symbol(&self, tags: Tag) -> RuntimeResult<SymbolNode>;
    fn get_symbols(&self, tags: &Vec<Tag>) -> RuntimeResult<Vec<SymbolNode>>;
}

impl NodeQuery for Interpreter {
    fn get_nodes(&self, tags: Vec<Tag>) -> RuntimeResult<Vec<Node>> {
        let mut nodes = vec![];
        for tag in tags {
            nodes.push(self.get_node(tag)?);
        }
        Ok(nodes)
    }

    fn get_node(&self, tag: Tag) -> RuntimeResult<Node> {
        self.ast
            .as_ref()
            .and_then(|ast| ast.get(&tag).cloned())
            .ok_or(ErrorKind::NodeNotFound)
    }

    fn get_vector(&self, tag: Tag) -> RuntimeResult<VectorNode> {
        if let Node::Vector(node) = self.get_node(tag)? {
            Ok(node)
        } else {
            Err(ErrorKind::MissingNode)
        }
    }

    fn get_symbol(&self, tag: Tag) -> RuntimeResult<SymbolNode> {
        if let Node::Symbol(node) = self.get_node(tag)? {
            Ok(node)
        } else {
            Err(ErrorKind::MissingNode)
        }
    }

    fn get_symbols(&self, tags: &Vec<Tag>) -> RuntimeResult<Vec<SymbolNode>> {
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
    pub fn new() -> Interpreter {
        let context = {
            use NativeFunction::*;
            let context = Context::new();
            context.new_namespace(Symbol::from("nomad.core"));
            context.define("now", Now);
            context.define("dump-context", DumpContext);
            context.define("=", Eq);
            context.define("mod", Mod);
            context.define("<", LessThan);
            // TODO: Convert to macro when that system is inplace.
            context.define("or", Or);
            context.define(">", GreaterThan);
            context.define("+", Plus);
            context.define("*", Multiply);
            context.define("/", Divide);
            context.define("-", Minus);
            context.define("get", Get);
            context.define("print", Print);
            context.define("println", Println);
            context.define("*version*", Value::make_number(0.into()));
            context
        };
        Interpreter {
            ast: None,
            context,
            values: Mutex::new(HashMap::new()),
        }
    }

    fn lt(&self, lhs: &Value, rhs: &Value) -> RuntimeResult<Value> {
        use Value::{Boolean, Number};
        match (lhs, rhs) {
            (Number(l), Number(r)) => Ok(l.lt(r).into()),
            pair => Err(ErrorKind::InvalidOperation),
        }
    }

    fn eq(&self, lhs: &Value, rhs: &Value) -> RuntimeResult<Value> {
        use Value::{Boolean, Number};
        match (lhs, rhs) {
            (Number(l), Number(r)) => Ok(l.eq(r).into()),
            pair => Err(ErrorKind::InvalidOperation),
        }
    }

    fn gt(&self, lhs: &Value, rhs: &Value) -> RuntimeResult<Value> {
        use Value::{Boolean, Number};
        match (lhs, rhs) {
            (Number(l), Number(r)) => Ok(l.gt(r).into()),
            pair => Err(ErrorKind::InvalidOperation),
        }
    }

    fn add(&self, lhs: &Value, rhs: &Value) -> RuntimeResult<Value> {
        use Value::{Number, String};
        match (lhs, rhs) {
            (Number(l), Number(r)) => Ok(Number(l.add(r))),
            (String(l), String(r)) => Ok(String(l.concat(r))),
            pair => Err(ErrorKind::InvalidOperation),
        }
    }

    fn modulus(&self, lhs: &Value, rhs: &Value) -> RuntimeResult<Value> {
        use Value::Number;
        match (lhs, rhs) {
            (Number(l), Number(r)) => Ok(Number(l.modulus(r))),
            pair => Err(ErrorKind::InvalidOperation),
        }
    }

    fn sub(&self, lhs: &Value, rhs: &Value) -> RuntimeResult<Value> {
        use Value::Number;
        match (lhs, rhs) {
            (Number(l), Number(r)) => Ok(Number(l.sub(r))),
            pair => Err(ErrorKind::InvalidOperation),
        }
    }

    fn mul(&self, lhs: &Value, rhs: &Value) -> RuntimeResult<Value> {
        use Value::Number;
        match (lhs, rhs) {
            (Number(l), Number(r)) => Ok(Number(l.mul(r))),
            pair => Err(ErrorKind::InvalidOperation),
        }
    }

    fn div(&self, lhs: &Value, rhs: &Value) -> RuntimeResult<Value> {
        use Value::Number;
        match (lhs, rhs) {
            (Number(l), Number(r)) => Ok(Number(l.div(r))),
            pair => Err(ErrorKind::InvalidOperation),
        }
    }

    pub fn dump_context(&self) {
        self.context.dump();
    }

    pub(crate) fn put(&self, symbol: value::Symbol, atom: value::Value) {
        let mut values = self.values.lock().unwrap();
        values.insert(symbol, atom);
    }

    pub fn interpret_tag(&self, tag: Tag) -> RuntimeResult<Value> {
        let node = self.get_node(tag)?;
        node.execute(&self)
    }

    pub fn resolve(&self, symbol: &Symbol) -> RuntimeResult<Value> {
        if let Ok(value) = self.context.get(symbol) {
            Ok(value)
        } else {
            self.context.resolve(symbol)
        }
    }

    pub fn interpret_and_resolve_tag(&self, tag: Tag) -> RuntimeResult<Value> {
        let value = self.interpret_tag(tag)?;
        Ok(value
            .as_symbol()
            .and_then(|symbol| self.resolve(symbol).ok())
            .unwrap_or(value))
    }

    pub fn root(&self) -> Option<Tag> {
        self.ast
            .as_ref()
            .and_then(|ast| ast.root)
            .map(|tag| tag.clone())
    }

    pub fn eval(&mut self, ast: AST) -> RuntimeResult<Value> {
        self.ast = Some(ast);
        if let Some(root) = self.root() {
            self.interpret_and_resolve_tag(root)
        } else {
            Ok(Value::Nil)
        }
    }

    pub fn define(&self, symbol: Symbol, value: Value) -> RuntimeResult<Value> {
        self.context.define(symbol.clone(), value).map(Value::Var)
    }

    pub fn set(&self, symbol: Symbol, value: Value) {
        self.context.set(symbol, value);
    }
}
