mod context;
use crate::result::RuntimeResult;
use context::Context;
mod value;
use execution::Execute;
use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use std::sync::Mutex;
use value::{Symbol, Value};

use crate::ast::{node, node::Node, parser::AST, Tag};
use crate::interpreter::value::NativeFunction;

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
            Symbol::from("<"),
            Value::NativeFunction(NativeFunction::LessThan),
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
        let mut queue = VecDeque::new();
        queue.push_back(ast.root.unwrap());
        Interpreter {
            ast,
            context,
            values: Mutex::new(HashMap::new()),
        }
    }

    fn lt(&self, lhs: &Value, rhs: &Value) -> Value {
        use Value::{Boolean, Number};
        match (lhs, rhs) {
            (Number(l), Number(r)) => Boolean(l < r),
            pair => panic!("lt not defined {:?}", pair),
        }
    }

    fn eq(&self, lhs: &Value, rhs: &Value) -> Value {
        use Value::{Boolean, Number};
        match (lhs, rhs) {
            (Number(l), Number(r)) => Boolean(l == r),
            pair => panic!("lt not defined {:?}", pair),
        }
    }

    fn gt(&self, lhs: &Value, rhs: &Value) -> Value {
        use Value::{Boolean, Number};
        match (lhs, rhs) {
            (Number(l), Number(r)) => Boolean(l > r),
            pair => panic!("gt not defined {:?}", pair),
        }
    }

    fn add(&self, lhs: &Value, rhs: &Value) -> Value {
        use Value::Number;
        match (lhs, rhs) {
            (Number(l), Number(r)) => Number(l + r),
            pair => panic!("add not defined {:?}", pair),
        }
    }

    fn sub(&self, lhs: &Value, rhs: &Value) -> Value {
        use Value::Number;
        match (lhs, rhs) {
            (Number(l), Number(r)) => Number(l - r),
            pair => panic!("add not defined {:?}", pair),
        }
    }

    fn mul(&self, lhs: &Value, rhs: &Value) -> Value {
        use Value::Number;
        match (lhs, rhs) {
            (Number(l), Number(r)) => Number(l * r),
            pair => panic!("add not defined {:?}", pair),
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

    pub fn get_node(&self, tag: Tag) -> Option<Node> {
        self.ast.get(&tag).cloned()
    }

    #[inline]
    pub(crate) fn interpret_tag(&self, tag: Tag) -> Value {
        let node = self.ast.get(&tag).unwrap();
        node.execute(&self)
    }

    pub fn interpret_and_resolve_tag(&self, tag: Tag) -> Value {
        let value = self.interpret_tag(tag);
        value
            .as_symbol()
            .and_then(|symbol| self.resolve(symbol).ok())
            .unwrap_or(value)
    }

    pub fn resolve(&self, symbol: &Symbol) -> RuntimeResult<Value> {
        Ok(self
            .context
            .get(symbol)
            .or_else(|| Some(self.context.resolve(symbol)))
            .unwrap())
    }

    pub fn define(&self, symbol: Symbol, value: Value) -> Value {
        Value::Var(self.context.define(symbol.clone(), value))
    }

    pub fn set(&self, symbol: Symbol, value: Value) {
        self.context.set(symbol, value);
    }

    fn run(&self) {
        if let Some(tag) = self.ast.root {
            self.interpret_tag(tag);
        }
    }
}

pub fn interpret(ast: AST) {
    let env = Interpreter::new(ast);
    env.run();
    // env.dump_context();
}

mod execution {
    use super::value::*;
    use super::Interpreter;
    use crate::ast::node::*;
    use crate::ast::tag::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    pub trait Execute {
        fn execute(&self, interpreter: &Interpreter) -> Value;
    }

    impl Execute for Node {
        fn execute(&self, interpreter: &Interpreter) -> Value {
            match self {
                Node::Boolean(node) => Value::Boolean(node.value()),
                Node::Definition(node) => node.execute(interpreter),
                Node::Do(node) => node.execute(interpreter),
                Node::Function(node) => node.execute(interpreter),
                Node::FunctionCall(node) => node.execute(interpreter),
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
                Node::Vector(node) => node.execute(interpreter),
                Node::While(node) => node.execute(interpreter),
                Node::Quote(node) => node.execute(interpreter),
                Node::Meta(node) => node.execute(interpreter),
                Node::Decorator(node) => node.execute(interpreter),
            }
        }
    }

    impl Execute for DecoratorNode {
        fn execute(&self, interpreter: &Interpreter) -> Value {
            todo!("decoration nodes are not defined");
        }
    }

    impl Execute for VectorNode {
        fn execute(&self, interpreter: &Interpreter) -> Value {
            todo!("Failed")
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

    impl Execute for QuoteNode {
        fn execute(&self, interpreter: &Interpreter) -> Value {
            interpreter.interpret_tag(self.expression())
        }
    }

    impl Execute for MetaNode {
        fn execute(&self, interpreter: &Interpreter) -> Value {
            todo!("meta data node not defined");
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

    impl Execute for FunctionCallNode {
        fn execute(&self, interpreter: &Interpreter) -> Value {
            let function = interpreter.interpret_and_resolve_tag(self.function());
            match function {
                Value::Function(function) => {
                    let parameters = interpreter
                        .get_node(function.parameters)
                        .and_then(Node::take_vector)
                        .expect("Paramters should be a vector node")
                        .items()
                        .into_iter()
                        .map(|tag| {
                            interpreter
                                .interpret_tag(tag)
                                .take_symbol()
                                .expect("Parameters should be symbols.")
                        })
                        .zip(
                            self.arguments()
                                .into_iter()
                                .map(|tag| interpreter.interpret_and_resolve_tag(tag)),
                        );

                    let mut result = Value::Nil;
                    for (s, v) in parameters {
                        interpreter.set(s, v);
                    }
                    // a new scope must be defined after the parameters are set
                    interpreter.context.push_scope();
                    for b in function.body {
                        result = interpreter.interpret_tag(b);
                    }
                    interpreter.context.pop_scope();
                    result
                }
                Value::NativeFunction(native) => match native {
                    NativeFunction::Now => {
                        let start = SystemTime::now();
                        let since_the_epoch = start
                            .duration_since(UNIX_EPOCH)
                            .expect("time went backwards");
                        Value::Number(since_the_epoch.as_millis() as f64)
                    }
                    NativeFunction::Plus => {
                        let mut arguments = self.arguments().into_iter();
                        match arguments.len() {
                            0 => Value::Number(0.0),
                            1 => interpreter.interpret_tag(arguments.next().unwrap()),
                            n => {
                                let mut base = interpreter
                                    .interpret_and_resolve_tag(arguments.next().expect("Name"));
                                for tag in arguments {
                                    let value = interpreter.interpret_and_resolve_tag(tag);
                                    base = interpreter.add(&base, &value);
                                }
                                base
                            }
                        }
                    }
                    NativeFunction::Minus => {
                        let mut arguments = self.arguments().into_iter();
                        match arguments.len() {
                            0 => Value::Number(0.0),
                            1 => interpreter.interpret_and_resolve_tag(arguments.next().unwrap()),
                            n => {
                                let mut base = interpreter
                                    .interpret_and_resolve_tag(arguments.next().expect("Name"));
                                for tag in arguments {
                                    let value = interpreter.interpret_and_resolve_tag(tag);
                                    base = interpreter.sub(&base, &value);
                                }
                                base
                            }
                        }
                    }
                    NativeFunction::Multiply => {
                        let mut arguments = self.arguments().into_iter();
                        match arguments.len() {
                            0 => Value::Number(1.0),
                            1 => interpreter.interpret_and_resolve_tag(arguments.next().unwrap()),
                            n => {
                                let mut base = interpreter
                                    .interpret_and_resolve_tag(arguments.next().expect("Name"));
                                for tag in arguments {
                                    let value = interpreter.interpret_and_resolve_tag(tag);
                                    base = interpreter.mul(&base, &value);
                                }
                                base
                            }
                        }
                    }
                    NativeFunction::DumpContext => {
                        interpreter.dump_context();
                        Value::Nil
                    }
                    NativeFunction::Print => {
                        let mut arguments = self.arguments().into_iter();
                        for (i, arg) in arguments.enumerate() {
                            let value = interpreter.interpret_and_resolve_tag(arg);
                            if i != 0 {
                                print!(" ");
                            }
                            print!("{}", value);
                        }
                        Value::Nil
                    }
                    NativeFunction::Println => {
                        let mut arguments = self.arguments().into_iter();
                        for (i, arg) in arguments.enumerate() {
                            let value = interpreter.interpret_and_resolve_tag(arg);
                            if i != 0 {
                                print!(" ");
                            }
                            print!("{}", value);
                        }
                        print!("\n");
                        Value::Nil
                    }
                    NativeFunction::Eq => {
                        let mut arguments = self.arguments().into_iter();
                        let mut flag = true;
                        let mut last: Option<Value> = None;
                        for arg in arguments {
                            let value = interpreter.interpret_and_resolve_tag(arg);
                            if let Some(l) = &last {
                                if interpreter.eq(&l, &value).is_truthy() {
                                    last = Some(value);
                                } else {
                                    flag = false;
                                    break;
                                }
                            } else {
                                last = Some(value);
                            }
                        }
                        Value::Boolean(flag)
                    }
                    NativeFunction::LessThan => {
                        let mut arguments = self.arguments().into_iter();
                        let mut flag = true;
                        let mut last: Option<Value> = None;
                        for arg in arguments {
                            let value = interpreter.interpret_and_resolve_tag(arg);
                            if let Some(l) = &last {
                                if interpreter.lt(&l, &value).is_truthy() {
                                    last = Some(value);
                                } else {
                                    flag = false;
                                    break;
                                }
                            } else {
                                last = Some(value);
                            }
                        }
                        Value::Boolean(flag)
                    }
                    NativeFunction::GreaterThan => {
                        let mut arguments = self.arguments().into_iter();
                        let mut flag = true;
                        let mut last: Option<Value> = None;
                        for arg in arguments {
                            let value = interpreter.interpret_and_resolve_tag(arg);
                            if let Some(l) = &last {
                                if interpreter.gt(&l, &value).is_truthy() {
                                    last = Some(value);
                                } else {
                                    flag = false;
                                    break;
                                }
                            } else {
                                last = Some(value);
                            }
                        }
                        Value::Boolean(flag)
                    }
                },
                value => panic!("{:?} is not callable", value),
            }
        }
    }

    impl Execute for FunctionNode {
        fn execute(&self, interpreter: &Interpreter) -> Value {
            self.name()
                .map(|name| {
                    let name = interpreter.interpret_tag(name);
                    let name = name.take_symbol().expect("Function names must be a symbol");
                    let value = Value::Function(Function {
                        parameters: self.parameters(),
                        body: self.body(),
                    });
                    // interpreter.define(name, value)
                    value
                })
                .unwrap_or_else(|| {
                    Value::Function(Function {
                        parameters: self.parameters(),
                        body: self.body(),
                    })
                })
        }
    }
}
