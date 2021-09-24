use super::ast::{node, node::Node, parser::AST, Tag};
use super::result::runtime::ErrorKind;
use super::result::RuntimeResult;
use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use std::sync::Mutex;

mod context;
mod execution;
mod value;

use context::Context;
use execution::Execute;
use value::{NativeFunction, Symbol, Value};

trait Operation {}

#[derive(Debug)]
pub struct Interpreter {
    ast: Option<AST>,
    context: Context,
    values: Mutex<HashMap<value::Symbol, value::Value>>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut context = Context::new();
        context.new_namespace(Symbol::from("nomad.core"));
        context.define(
            Symbol::from("now"),
            Value::NativeFunction(NativeFunction::Now),
        );
        context.define(
            Symbol::from("dump-context"),
            Value::NativeFunction(NativeFunction::DumpContext),
        );
        context.define(Symbol::from("="), Value::NativeFunction(NativeFunction::Eq));
        context.define(
            Symbol::from("mod"),
            Value::NativeFunction(NativeFunction::Mod),
        );
        context.define(
            Symbol::from("<"),
            Value::NativeFunction(NativeFunction::LessThan),
        );
        context.define(
            Symbol::from("or"),
            Value::NativeFunction(NativeFunction::Or),
        );
        context.define(
            Symbol::from(">"),
            Value::NativeFunction(NativeFunction::GreaterThan),
        );
        context.define(
            Symbol::from("+"),
            Value::NativeFunction(NativeFunction::Plus),
        );
        context.define(
            Symbol::from("*"),
            Value::NativeFunction(NativeFunction::Multiply),
        );
        context.define(
            Symbol::from("/"),
            Value::NativeFunction(NativeFunction::Divide),
        );
        context.define(
            Symbol::from("-"),
            Value::NativeFunction(NativeFunction::Minus),
        );
        context.define(
            Symbol::from("print"),
            Value::NativeFunction(NativeFunction::Print),
        );
        context.define(
            Symbol::from("println"),
            Value::NativeFunction(NativeFunction::Println),
        );
        context.define(
            Symbol::from("*version*"),
            0.into(),
        );
        Interpreter {
            ast: None,
            context,
            values: Mutex::new(HashMap::new()),
        }
    }

    fn lt(&self, lhs: &Value, rhs: &Value) -> RuntimeResult<Value> {
        use Value::{Boolean, Number};
        match (lhs, rhs) {
            (Number(l), Number(r)) => Ok(Boolean(l < r)),
            pair => Err(ErrorKind::InvalidOperation),
        }
    }

    fn eq(&self, lhs: &Value, rhs: &Value) -> RuntimeResult<Value> {
        use Value::{Boolean, Number};
        match (lhs, rhs) {
            (Number(l), Number(r)) => Ok(Boolean(l == r)),
            pair => Err(ErrorKind::InvalidOperation),
        }
    }

    fn gt(&self, lhs: &Value, rhs: &Value) -> RuntimeResult<Value> {
        use Value::{Boolean, Number};
        match (lhs, rhs) {
            (Number(l), Number(r)) => Ok(Boolean(l > r)),
            pair => Err(ErrorKind::InvalidOperation),
        }
    }

    fn add(&self, lhs: &Value, rhs: &Value) -> RuntimeResult<Value> {
        use Value::Number;
        match (lhs, rhs) {
            (Number(l), Number(r)) => Ok(Number(l + r)),
            (Value::String(l), Value::String(r)) => Ok(Value::String(format!("{}{}", l, r))),
            pair => Err(ErrorKind::InvalidOperation),
        }
    }

    fn modulus(&self, lhs: &Value, rhs: &Value) -> RuntimeResult<Value> {
        use Value::Number;
        match (lhs, rhs) {
            (Number(l), Number(r)) => Ok(Number(l % r)),
            pair => Err(ErrorKind::InvalidOperation),
        }
    }

    fn sub(&self, lhs: &Value, rhs: &Value) -> RuntimeResult<Value> {
        use Value::Number;
        match (lhs, rhs) {
            (Number(l), Number(r)) => Ok(Number(l - r)),
            pair => Err(ErrorKind::InvalidOperation),
        }
    }

    fn mul(&self, lhs: &Value, rhs: &Value) -> RuntimeResult<Value> {
        use Value::Number;
        match (lhs, rhs) {
            (Number(l), Number(r)) => Ok(Number(l * r)),
            pair => Err(ErrorKind::InvalidOperation),
        }
    }

    fn div(&self, lhs: &Value, rhs: &Value) -> RuntimeResult<Value> {
        use Value::Number;
        match (lhs, rhs) {
            (Number(l), Number(r)) => Ok(Number(l / r)),
            pair => Err(ErrorKind::InvalidOperation),
        }
    }

    pub fn dump_context(&self) {
        self.context.dump();
    }

    #[inline]
    pub(crate) fn put(&self, symbol: value::Symbol, atom: value::Value) {
        let mut values = self.values.lock().unwrap();
        values.insert(symbol, atom);
    }

    pub fn get_node(&self, tag: Tag) -> RuntimeResult<Node> {
        self.ast
            .as_ref()
            .and_then(|ast| ast.get(&tag).cloned())
            .ok_or(ErrorKind::NodeNotFound)
    }

    #[inline]
    pub fn interpret_tag(&self, tag: Tag) -> RuntimeResult<Value> {
        let node = self.get_node(tag)?;
        node.execute(&self)
    }

    pub fn resolve(&self, symbol: &Symbol) -> RuntimeResult<Value> {
        // I'm not happy with this implementation
        Ok(self
            .context
            .get(symbol)
            .or_else(|| Some(self.context.resolve(symbol)))
            .unwrap())
    }

    #[inline]
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
        Ok(Value::Var(self.context.define(symbol.clone(), value)))
    }

    pub fn set(&self, symbol: Symbol, value: Value) {
        self.context.set(symbol, value);
    }
}
