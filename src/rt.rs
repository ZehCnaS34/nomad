use crate::ast::{is_qualified, name, namespace, split_symbol, Number, Symbol, Value};
use crate::result::{runtime_issue, NResult};
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug)]
pub struct Namespace {
    pub name: Symbol,
    pub aliases: HashMap<Symbol, Symbol>,
    pub bindings: HashMap<String, Value>,
}

impl Namespace {
    fn new<T: Into<Symbol>>(name: T) -> Namespace {
        let mut ns = Namespace {
            name: name.into(),
            aliases: HashMap::new(),
            bindings: HashMap::new(),
        };
        ns
    }

    fn get(&self, symbol: &Symbol) -> NResult<&Value> {
        match self.bindings.get(symbol) {
            Some(value) => Ok(value),
            None => runtime_issue("Symbol not defined"),
        }
    }

    fn define(&mut self, symbol: &Symbol, value: Value) {
        self.bindings.insert(symbol.clone(), value);
    }
}

#[derive(Debug)]
pub struct Runtime {
    pub namespaces: RefCell<HashMap<Symbol, Namespace>>,
    pub current_namespace: Symbol,
}

pub enum RuntimeError {
    Generic,
}

const ERR: RuntimeError = RuntimeError::Generic;

impl Runtime {
    pub fn new() -> Runtime {
        let name = "nomad.core";
        let namespace = Namespace::new(name);
        let namespaces = RefCell::new(HashMap::new());
        {
            let mut namespaces = namespaces.borrow_mut();
            namespaces.insert(name.into(), namespace);
        }
        Runtime {
            namespaces,
            current_namespace: name.into(),
        }
    }

    pub fn resolve<Action>(&self, symbol: &Symbol, action: Action) -> NResult<Value>
    where
        Action: Fn(&Value) -> NResult<Value>,
    {
        let (ns, n) = if is_qualified(symbol) {
            (namespace(symbol), name(symbol))
        } else {
            (self.current_namespace.clone(), symbol.clone())
        };
        let namespaces = self.namespaces.borrow();
        let namespace = match namespaces.get(&ns) {
            Some(namespace) => Ok(namespace),
            None => runtime_issue("Failed"),
        }?;
        let value = namespace.get(&n)?;
        // match namespace.bindings.get(&name(&symbol)) {
        //     Some()
        // }
        action(&value)
    }

    pub fn define(&self, name: Symbol, value: Option<Value>) -> NResult<()> {
        let mut namespaces = self.namespaces.borrow_mut();
        let namespace = match namespaces.get_mut(&self.current_namespace) {
            Some(namespace) => Ok(namespace),
            None => runtime_issue("Namespaces does not exist"),
        }?;
        namespace.define(&name, value.unwrap());
        Ok(())
    }

    pub fn add(&self, a: &Value, b: &Value) -> NResult<Value> {
        use Value::*;
        match (a, b) {
            (Number(a), Number(b)) => Ok(Number(a + b)),
            (String(a), String(b)) => Ok(String(format!("{}{}", a, b))),
            (Symbol(a), _) => self.resolve(&a, |a| self.add(a, b)),
            (_, Symbol(b)) => self.resolve(&b, |b| self.add(a, b)),
            _ => runtime_issue("Cannot run this.."),
        }
    }

    pub fn sub(&self, a: &Value, b: &Value) -> NResult<Value> {
        use Value::*;
        match (a, b) {
            (Number(a), Number(b)) => Ok(Number(a - b)),
            (Symbol(a), _) => self.resolve(&a, |a| self.add(a, b)),
            (_, Symbol(b)) => self.resolve(&b, |b| self.add(a, b)),
            _ => runtime_issue("Cannot run this.."),
        }
    }
}
