use crate::ast::{Expr, Symbol, SymbolIntrospection, Value};
use crate::result::{issue, runtime_issue, NResult};
use std::cell::RefCell;
use std::collections::HashMap;
use std::borrow::BorrowMut;

fn take_2<T, S: Clone + Into<String>>(args: Vec<T>, s: S) -> NResult<(T, T)> {
    let mut args = args.into_iter();
    let one = args.next().ok_or(issue(format!(
        "{} requires Two arguments. Zero given.",
        s.clone().into()
    )))?;
    let two = args.next().ok_or(issue(format!(
        "{} requires Two Arguments. One given.",
        s.clone().into()
    )))?;
    Ok((one, two))
}

fn take_2_maybe_3<T, S: Clone + Into<String>>(args: Vec<T>, s: S) -> NResult<(T, T, Option<T>)> {
    let mut args = args.into_iter();
    let one = args.next().ok_or(issue(format!(
        "{} requires Two arguments. Zero given.",
        s.clone().into()
    )))?;
    let two = args.next().ok_or(issue(format!(
        "{} requires Two Arguments. One given.",
        s.clone().into()
    )))?;
    let three = args.next();
    Ok((one, two, three))
}

fn take_3<T, S: Clone + Into<String>>(args: Vec<T>, s: S) -> NResult<(T, T, T)> {
    let mut args = args.into_iter();
    let one = args.next().ok_or(issue(format!(
        "{} requires Three arguments. Zero given.",
        s.clone().into()
    )))?;
    let two = args.next().ok_or(issue(format!(
        "{} requires Three Arguments. One given.",
        s.clone().into()
    )))?;
    let three = args.next().ok_or(issue(format!(
        "{} requires Three Arguments. Two given.",
        s.clone().into()
    )))?;
    Ok((one, two, three))
}

const BLOCK: &'static str = "nomad.core/block";
const DEF: &'static str = "nomad.core/def";
const DIVIDE: &'static str = "nomad.core//";
// this looks ugly
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
    pub parent: Option<Symbol>,
    pub name: Symbol,
    pub aliases: HashMap<Symbol, Symbol>,
    pub bindings: HashMap<String, Value>,
    pub locals: HashMap<Symbol, Value>,
}

impl Namespace {
    fn new<S: Into<String>>(name: S) -> Namespace {
        Namespace {
            parent: None,
            name: name.into(),
            aliases: HashMap::new(),
            bindings: HashMap::new(),
            locals: HashMap::new(),
        }
    }
    fn core() -> Namespace {
        Namespace::new("nomad.core")
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

    fn ns_count(&self) -> usize {
        self.namespaces.borrow().len()
    }

    fn push_scope(&self) -> NResult<()> {
        let count = self.ns_count();
        self.using_namespace(self.current_namespace(), |parent| {
            let child_name = format!("{}.scope_{}", parent.name.clone(), count);
            let child = Namespace::new(child_name);
            let mut namespaces = self.namespaces.borrow_mut();
            namespaces.insert(0, child);
            Ok(Value::Nil)
        }).and(Ok(()))
    }

    fn pop_scope(&self) {
        let mut namespaces = self.namespaces.borrow_mut();
        namespaces.remove(0);
    }

    pub fn using_namespace<Action>(&self, ns: Symbol, action: Action) -> NResult<Value>
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

    pub fn using_mut_namespace<Action>(&self, ns: Symbol, mut action: Action) -> NResult<Value>
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
        self.using_namespace(ns, |namespace| {
            let value = namespace
                .bindings
                .get(&n)
                .ok_or(issue(format!("Failed to resolve binding {:?}", n)))?;
            action(value.clone())
        })
    }

    pub fn define(&self, name: Symbol, value: Option<Value>) -> NResult<Value> {
        let (ns, n) = self.inflate_symbol(name);
        self.using_mut_namespace(ns, move |namespace| {
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
            (Number(a), Number(b)) => Ok(Boolean(a > b)),
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
            (Number(a), Number(b)) => Ok(Number(a * b)),
            (Symbol(a), b) => self.resolve(a, |a| self.mult(a, b.clone())),
            (a, Symbol(b)) => self.resolve(b, |b| self.mult(a.clone(), b)),
            _ => runtime_issue("Cannot run this.."),
        }
    }

    pub fn div(&self, a: Value, b: Value) -> NResult<Value> {
        use Value::*;
        match (a, b) {
            (Number(a), Number(b)) => Ok(Number(a / b)),
            (Symbol(a), b) => self.resolve(a, |a| self.div(a, b.clone())),
            (a, Symbol(b)) => self.resolve(b, |b| self.div(a.clone(), b)),
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
                    .take_symbol()
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
                        let (left, right) = take_2(args, "Mod")?;
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
                    DIVIDE => {
                        let mut args = args.into_iter();
                        let mut difference = self.interpret(
                            args.next()
                                .ok_or(issue("Division requires at least one argument."))?,
                        )?;
                        for operand in args {
                            difference = self.div(difference, self.interpret(operand)?)?;
                        }
                        Ok(difference)
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
                            .interpret(args.next().ok_or(issue("Arguments for > are required"))?)?;
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
                        let mut args = args.into_iter();
                        let mut difference = self.interpret(
                            args.next()
                                .ok_or(issue("Subtraction requires at least one argument."))?,
                        )?;
                        for operand in args {
                            difference = self.sub(difference, self.interpret(operand)?)?;
                        }
                        Ok(difference)
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
                        let (name, value) = take_2(args, "Def")?;
                        let name = name
                            .take_atom()?
                            .take_symbol()
                            .or(runtime_issue("Slot one of def should be a symbol."))?;
                        let value = self.interpret(value)?;
                        self.define(name, Some(value))
                    }
                    IF => {
                        let (condition, true_branch, false_branch) = take_2_maybe_3(args, "If")?;
                        let condition = self.interpret(condition)?;
                        if condition.is_truthy() {
                            self.interpret(true_branch)
                        } else if let Some(branch) = false_branch {
                            self.interpret(branch)
                        } else {
                            Ok(Value::Nil)
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
