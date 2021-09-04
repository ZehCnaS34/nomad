use crate::ast::{is_qualified, name, namespace, Expr, Symbol, Value};
use crate::result::{issue, runtime_issue, NResult};
use std::cell::RefCell;
use std::collections::HashMap;

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
        ns.define("+".into(), Value::Symbol("nomad.core/+".into()));
        ns.define("-".into(), Value::Symbol("nomad.core/-".into()));
        ns.define("<".into(), Value::Symbol("nomad.core/<".into()));
        ns.define("=".into(), Value::Symbol("nomad.core/=".into()));
        ns.define("println".into(), Value::Symbol("nomad.core/println".into()));
        ns.define("def".into(), Value::Symbol("nomad.core/def".into()));
        ns.define("while".into(), Value::Symbol("nomad.core/while".into()));
        ns.define("if".into(), Value::Symbol("nomad.core/if".into()));
        ns
    }

    fn get(&self, symbol: &Symbol) -> NResult<&Value> {
        match self.bindings.get(symbol) {
            Some(value) => Ok(value),
            None => runtime_issue("Symbol not defined"),
        }
    }

    fn define(&mut self, symbol: Symbol, value: Value) {
        self.bindings.insert(symbol.clone(), value);
    }
}

#[derive(Debug)]
pub struct Runtime {
    pub namespaces: RefCell<HashMap<Symbol, Namespace>>,
    pub current_namespace: Symbol,
}

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

    pub fn resolve<Action>(&self, symbol: Symbol, action: Action) -> NResult<Value>
    where
        Action: Fn(Value) -> NResult<Value>,
    {
        let (ns, n) = if is_qualified(&symbol) {
            (namespace(&symbol), name(&symbol))
        } else {
            (self.current_namespace.clone(), symbol.clone())
        };
        let namespaces = self.namespaces.borrow();
        let namespace = match namespaces.get(&ns) {
            Some(namespace) => Ok(namespace),
            None => runtime_issue("Failed"),
        }?;
        let value = namespace.get(&n)?.clone();
        // match namespace.bindings.get(&name(&symbol)) {
        //     Some()
        // }
        action(value)
    }

    pub fn define(&self, name: Symbol, value: Option<Value>) -> NResult<Value> {
        let mut namespaces = self.namespaces.borrow_mut();
        let namespace = match namespaces.get_mut(&self.current_namespace) {
            Some(namespace) => Ok(namespace),
            None => runtime_issue("Namespaces does not exist"),
        }?;
        namespace.define(name, value.unwrap());
        Ok(Value::Nil)
    }

    pub fn eq(&self, a: Value, b: Value) -> NResult<Value> {
        use Value::*;
        match (a, b) {
            (String(a), String(b)) => Ok(Boolean(a == b)),
            (Number(a), Number(b)) => Ok(Boolean(a == b)),
            (Boolean(a), Boolean(b)) => Ok(Boolean(a == b)),
            (Nil, Nil) => Ok(Boolean(true)),
            (Symbol(a), b) => self.resolve(a, |a| self.eq(a, b.clone())),
            (a, Symbol(b)) => self.resolve(b, |b| self.eq(a.clone(), b)),
            _ => runtime_issue("Cannot run this.."),
        }
    }

    pub fn lt(&self, a: Value, b: Value) -> NResult<Value> {
        use Value::*;
        match (a, b) {
            (Number(a), Number(b)) => Ok(Boolean(a < b)),
            (Symbol(a), b) => self.resolve(a, |a| self.lt(a, b.clone())),
            (a, Symbol(b)) => self.resolve(b, |b| self.lt(a.clone(), b)),
            _ => runtime_issue("Cannot run this.."),
        }
    }

    pub fn add(&self, a: Value, b: Value) -> NResult<Value> {
        use Value::*;
        match (a, b) {
            (Number(a), Number(b)) => Ok(Number(a + b)),
            (String(a), String(b)) => Ok(String(format!("{}{}", a, b))),
            (Symbol(a), b) => self.resolve(a, |a| self.add(a, b.clone())),
            (a, Symbol(b)) => self.resolve(b, |b| self.add(a.clone(), b)),
            _ => runtime_issue("Cannot run this.."),
        }
    }

    pub fn sub(&self, a: Value, b: Value) -> NResult<Value> {
        use Value::*;
        match (a, b) {
            (Number(a), Number(b)) => Ok(Number(a - b)),
            (Symbol(a), b) => self.resolve(a, |a| self.sub(a, b.clone())),
            (a, Symbol(b)) => self.resolve(b, move |b| self.sub(a.clone(), b)),
            _ => runtime_issue("Cannot run this.."),
        }
    }

    pub fn interpret(&self, v: Expr) -> NResult<Value> {
        match v {
            Expr::Atom(value) => match value {
                Value::Symbol(symbol) => self.resolve(symbol, |value| Ok(value)),
                value => Ok(value),
            },
            Expr::Program(expressions) => {
                let mut result = Value::Nil;
                for expression in expressions {
                    result = self.interpret(expression)?;
                }
                Ok(result)
            }
            Expr::Invoke(f, args) => {
                let f = self.interpret(*f)?.get_symbol().or(Err(issue("Must resolve to something invokable")))?;
                match f.as_str() {
                    "nomad.core/+" => {
                        let mut sum = Value::Number(0.0);
                        for operand in args {
                            sum = self.add(sum, self.interpret(operand)?)?;
                        }
                        Ok(sum)
                    }
                    "nomad.core/<" => {
                        let mut args = args.into_iter();
                        let mut flag = true;
                        let mut last = self.interpret(args.next().ok_or(issue("Arguments for < are required"))?)?;
                        while let Some(next) = args.next() {
                            let next = self.interpret(next)?;
                            let check = self.lt(last.clone(), next.clone())?;
                            if !check.is_truthy() {
                                flag = false;
                                break;
                            } else {
                                last = next;
                            }
                        }
                        Ok(Value::Boolean(flag))
                    }
                    "nomad.core/=" => {
                        let mut args = args.into_iter();
                        let mut flag = true;
                        let mut last = self.interpret(args.next().ok_or(issue("Arguments for = are required"))?)?;
                        while let Some(next) = args.next() {
                            let next = self.interpret(next)?;
                            let check = self.eq(last.clone(), next.clone())?;
                            if !check.is_truthy() {
                                flag = false;
                                break;
                            } else {
                                last = next;
                            }
                        }
                        Ok(Value::Boolean(flag))
                    }
                    "nomad.core/-" => {
                        let mut sum = Value::Number(0.0);
                        for operand in args {
                            sum = self.sub(sum, self.interpret(operand)?)?;
                        }
                        Ok(sum)
                    }
                    "nomad.core/while" => {
                        let mut args = args.into_iter();
                        let condition = args.next().ok_or(issue("While loops must contain a condition"))?;
                        let body: Vec<_> = args.collect();
                        loop {
                            let check = self.interpret(condition.clone())?;
                            if !check.is_truthy() {
                                break;
                            } 
                            for expr in body.clone() {
                                self.interpret(expr)?;
                            }
                        }
                        Ok(Value::Nil)
                    }
                    "nomad.core/println" => {
                        let size = args.len();
                        for (i, arg) in args.into_iter().enumerate() {
                            let value = self.interpret(arg)?;
                            print!("{}", value);
                            if i + 1 != size {
                                print!(" ");
                            }
                        }
                        print!("\n");
                        Ok(Value::Nil)
                    }
                    "nomad.core/def" => {
                        let mut args = args.into_iter();
                        let name = if let Some(name) = args.next() {
                            name.get_atom()?.get_symbol()?
                        } else {
                            return runtime_issue("Invalid def name");
                        };

                        let value = if let Some(expr) = args.next() {
                            self.interpret(expr)?
                        } else {
                            return runtime_issue("def missing initializer");
                        };

                        self.define(name, Some(value))
                    }
                    "nomad.core/if" => {
                        let mut args = args.into_iter();
                        let condition = if let Some(condition) = args.next() {
                            self.interpret(condition)?
                        } else {
                            return runtime_issue("Failed to parse condition");
                        };

                        if condition.is_truthy() {
                            if let Some(branch) = args.next() {
                                self.interpret(branch)
                            } else {
                                runtime_issue("Failed to parse condition")
                            }
                        } else {
                            if let Some(branch) = args.skip(1).next() {
                                self.interpret(branch)
                            } else {
                                Ok(Value::Nil)
                                // runtime_issue("Failed to parse condition")
                            }
                        }
                    }
                    value => {
                        println!("value = {}", value);
                        todo!()
                    }
                }
            }
            _ => todo!(),
        }
    }
}
