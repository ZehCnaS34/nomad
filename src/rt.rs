use crate::ast::{Expr, Symbol, SymbolIntrospection, Value};
use crate::result::{issue, runtime_issue, NResult};
use std::cell::RefCell;
use std::collections::HashMap;

const BLOCK: &'static str = "nomad.core/block";
const DEF: &'static str = "nomad.core/def";
const DIVIDE: &'static str = "nomad.core//";
const DO: &'static str = "nomad.core/do";
const EQ: &'static str = "nomad.core/=";
const GT: &'static str = "nomad.core/>";
const IF: &'static str = "nomad.core/if";
const LET: &'static str = "nomad.core/let";
const LT: &'static str = "nomad.core/<";
const PLUS: &'static str = "nomad.core/+";
const MINUS: &'static str = "nomad.core/-";
const MULT: &'static str = "nomad.core/*";
const MOD: &'static str = "nomad.core/mod";
const PRINTLN: &'static str = "nomad.core/println";
const WHILE: &'static str = "nomad.core/while";

pub struct Scope {
    parents: Vec<Scope>,
    local: RefCell<HashMap<String, Value>>,
}

impl Scope {
    fn new() -> Scope {
        Scope {
            parents: vec![],
            local: RefCell::new(HashMap::new()),
        }
    }
}

#[derive(Debug)]
pub struct Namespace {
    pub parent: Option<Box<Namespace>>,
    pub name: Symbol,
    pub aliases: HashMap<Symbol, Symbol>,
    pub bindings: HashMap<String, Value>,
}

impl Namespace {
    fn core() -> Namespace {
        Namespace {
            parent: None,
            name: "nomad.core".into(),
            aliases: HashMap::new(),
            bindings: HashMap::new(),
        }
        .define_str(BLOCK)
        .define_str(DEF)
        .define_str(DIVIDE)
        .define_str(DO)
        .define_str(EQ)
        .define_str(GT)
        .define_str(IF)
        .define_str(LET)
        .define_str(LT)
        .define_str(MINUS)
        .define_str(MOD)
        .define_str(MULT)
        .define_str(PLUS)
        .define_str(PRINTLN)
        .define_str(WHILE)
    }

    fn get(&self, symbol: &Symbol) -> NResult<&Value> {
        match self.bindings.get(symbol) {
            Some(value) => Ok(value),
            None => runtime_issue("Symbol not defined"),
        }
    }

    fn define_str<S: Into<Symbol>>(mut self, s: S) -> Self {
        let symbol = s.into();
        if !symbol.is_qualified() {
            panic!("Failed");
        }
        let (_, n) = symbol.split_symbol().unwrap();
        self.define(n, Value::Symbol(symbol));
        self
    }

    fn define(&mut self, symbol: Symbol, value: Value) {
        self.bindings.insert(symbol.clone(), value);
    }
}

#[derive(Debug)]
pub struct Runtime {
    pub namespaces: RefCell<Vec<Namespace>>,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            namespaces: RefCell::new(vec![Namespace::core()]),
        }
    }

    fn current_namespace(&self) -> Symbol {
        let namespaces = self.namespaces.borrow_mut();
        let namespace = namespaces
            .first()
            .expect("The namespaces vector should never be empty");
        namespace.name.clone()
    }

    fn push_scope(&mut self) {}

    fn pop_scope(&mut self) {}

    pub fn with_namespace<Action>(&self, ns: Symbol, action: Action) -> NResult<Value>
    where
        Action: Fn(&Namespace) -> NResult<Value>,
    {
        let namespaces = self.namespaces.borrow();
        for namespace in namespaces.iter() {
            if namespace.name == ns {
                return action(namespace);
            }
        }
        Err(issue("failed to resolve namespace"))
    }

    pub fn with_mut_namespace<Action>(&self, ns: Symbol, mut action: Action) -> NResult<Value>
    where
        Action: FnMut(&mut Namespace) -> NResult<Value>,
    {
        let mut namespaces = self.namespaces.borrow_mut();
        for namespace in namespaces.iter_mut() {
            if namespace.name == ns {
                return action(namespace);
            }
        }
        Err(issue("failed to resolve namespace"))
    }

    pub fn inflate_symbol(&self, symbol: Symbol) -> (Symbol, Symbol) {
        if let Some(ns) = symbol.namespace() {
            (ns, symbol.name())
        } else {
            (self.current_namespace(), symbol)
        }
    }

    pub fn resolve<Action>(&self, symbol: Symbol, action: Action) -> NResult<Value>
    where
        Action: Fn(Value) -> NResult<Value>,
    {
        let (ns, n) = self.inflate_symbol(symbol);
        self.with_namespace(ns, |namespace| {
            let value = namespace
                .bindings
                .get(&n)
                .ok_or(issue("Failed to resolve binding"))?;
            action(value.clone())
        })
    }

    pub fn define(&self, name: Symbol, value: Option<Value>) -> NResult<Value> {
        let (ns, n) = self.inflate_symbol(name);
        self.with_mut_namespace(ns, move |namespace| {
            namespace.define(n.clone(), value.clone().unwrap());
            Ok(Value::Nil)
        })
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

    pub fn gt(&self, a: Value, b: Value) -> NResult<Value> {
        use Value::*;
        match (a, b) {
            (Number(a), Number(b)) => Ok(Boolean(a < b)),
            (Symbol(a), b) => self.resolve(a, |a| self.gt(a, b.clone())),
            (a, Symbol(b)) => self.resolve(b, |b| self.gt(a.clone(), b)),
            _ => runtime_issue("Cannot run this.."),
        }
    }

    pub fn modu(&self, a: Value, b: Value) -> NResult<Value> {
        use Value::*;
        match (a, b) {
            (Number(a), Number(b)) => Ok(Number(a % b)),
            (Symbol(a), b) => self.resolve(a, |a| self.modu(a, b.clone())),
            (a, Symbol(b)) => self.resolve(b, |b| self.modu(a.clone(), b)),
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

    pub fn mult(&self, a: Value, b: Value) -> NResult<Value> {
        use Value::*;
        match (a, b) {
            (Number(a), Number(b)) => Ok(Number(a + b)),
            (Symbol(a), b) => self.resolve(a, |a| self.mult(a, b.clone())),
            (a, Symbol(b)) => self.resolve(b, |b| self.mult(a.clone(), b)),
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
                let f = self
                    .interpret(*f)?
                    .get_symbol()
                    .or(Err(issue("Must resolve to something invocable")))?;
                match f.as_str() {
                    PLUS => {
                        let mut sum = Value::Number(0.0);
                        for operand in args {
                            sum = self.add(sum, self.interpret(operand)?)?;
                        }
                        Ok(sum)
                    }
                    MOD => {
                        let mut args = args.into_iter();
                        let left = args.next().ok_or(issue("Mod requires 2 arguments. None given"))?;
                        let right = args.next().ok_or(issue("Mod required 2 arguments. 1 given"))?;
                        let left = self.interpret(left)?;
                        let right = self.interpret(right)?;
                        self.modu(left, right)
                    }
                    MULT => {
                        let mut sum = Value::Number(1.0);
                        for operand in args {
                            sum = self.mult(sum, self.interpret(operand)?)?;
                        }
                        Ok(sum)
                    }
                    LT => {
                        let mut args = args.into_iter();
                        let mut flag = true;
                        let mut last = self
                            .interpret(args.next().ok_or(issue("Arguments for < are required"))?)?;
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
                    GT => {
                        let mut args = args.into_iter();
                        let mut flag = true;
                        let mut last = self
                            .interpret(args.next().ok_or(issue("Arguments for < are required"))?)?;
                        while let Some(next) = args.next() {
                            let next = self.interpret(next)?;
                            let check = self.gt(last.clone(), next.clone())?;
                            if !check.is_truthy() {
                                flag = false;
                                break;
                            } else {
                                last = next;
                            }
                        }
                        Ok(Value::Boolean(flag))
                    }
                    EQ => {
                        let mut args = args.into_iter();
                        let mut flag = true;
                        let mut last = self
                            .interpret(args.next().ok_or(issue("Arguments for = are required"))?)?;
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
                    DO => {
                        let mut args = args.into_iter();
                        let mut result = Value::Nil;
                        while let Some(next) = args.next() {
                            result = self.interpret(next)?;
                        }
                        Ok(result)
                    }
                    BLOCK => {
                        let mut args = args.into_iter();
                        let mut result = Value::Nil;
                        while let Some(next) = args.next() {
                            result = self.interpret(next)?;
                        }
                        Ok(result)
                    }
                    MINUS => {
                        let mut sum = Value::Number(0.0);
                        for operand in args {
                            sum = self.sub(sum, self.interpret(operand)?)?;
                        }
                        Ok(sum)
                    }
                    WHILE => {
                        let mut args = args.into_iter();
                        let condition = args
                            .next()
                            .ok_or(issue("While loops must contain a condition"))?;
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
                    PRINTLN => {
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
                    DEF => {
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
                    IF => {
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
