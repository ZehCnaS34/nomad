use super::value::*;
use super::Interpreter;
use crate::ast::node::*;
use crate::ast::tag::*;
use crate::result::runtime::ErrorKind;
use crate::result::RuntimeResult;
use std::time::{SystemTime, UNIX_EPOCH};

pub trait Execute {
    fn execute(&self, interpreter: &Interpreter) -> RuntimeResult<Value>;
}

impl Execute for Node {
    fn execute(&self, interpreter: &Interpreter) -> RuntimeResult<Value> {
        match self {
            Node::Boolean(node) => Ok(Value::Boolean(node.value())),
            Node::Decorator(node) => node.execute(interpreter),
            Node::Definition(node) => node.execute(interpreter),
            Node::Do(node) => node.execute(interpreter),
            Node::Function(node) => node.execute(interpreter),
            Node::FunctionCall(node) => node.execute(interpreter),
            Node::If(node) => node.execute(interpreter),
            Node::Keyword(..) => todo!("keywords are no done"),
            Node::Let(node) => todo!("need to implement let"),
            Node::List(node) => todo!(),
            Node::Loop(node) => todo!(),
            Node::Meta(node) => node.execute(interpreter),
            Node::Nil => Ok(Value::Nil),
            Node::Number(number) => Ok(Value::Number(number.value())),
            Node::Program(node) => node.execute(interpreter),
            Node::Quote(node) => node.execute(interpreter),
            Node::Recur(node) => todo!(),
            Node::String(node) => Ok(Value::String(String::from(node.value()))),
            Node::Symbol(node) => Ok(Value::Symbol(Symbol::from_node(node.clone()))),
            Node::Vector(node) => node.execute(interpreter),
            Node::While(node) => node.execute(interpreter),
        }
    }
}

impl Execute for DecoratorNode {
    fn execute(&self, interpreter: &Interpreter) -> RuntimeResult<Value> {
        todo!("decoration nodes are not defined");
    }
}

impl Execute for VectorNode {
    fn execute(&self, interpreter: &Interpreter) -> RuntimeResult<Value> {
        todo!("Failed")
    }
}

impl Execute for ProgramNode {
    fn execute(&self, interpreter: &Interpreter) -> RuntimeResult<Value> {
        let mut result = Value::Nil;
        for tag in self.expressions() {
            result = interpreter.interpret_tag(tag)?;
        }
        Ok(result)
    }
}

impl Execute for IfNode {
    fn execute(&self, interpreter: &Interpreter) -> RuntimeResult<Value> {
        if interpreter
            .interpret_and_resolve_tag(self.condition)?
            .is_truthy()
        {
            interpreter.interpret_and_resolve_tag(self.true_branch)
        } else {
            interpreter.interpret_and_resolve_tag(self.false_branch)
        }
    }
}

impl Execute for QuoteNode {
    fn execute(&self, interpreter: &Interpreter) -> RuntimeResult<Value> {
        interpreter.interpret_tag(self.expression())
    }
}

impl Execute for MetaNode {
    fn execute(&self, interpreter: &Interpreter) -> RuntimeResult<Value> {
        todo!("meta data node not defined");
    }
}

impl Execute for WhileNode {
    fn execute(&self, interpreter: &Interpreter) -> RuntimeResult<Value> {
        loop {
            let condition = interpreter.interpret_tag(self.condition())?;
            if !condition.is_truthy() {
                break;
            }
            for tag in self.body() {
                interpreter.interpret_tag(tag)?;
            }
        }
        Ok(Value::Nil)
    }
}

impl Execute for DefinitionNode {
    fn execute(&self, interpreter: &Interpreter) -> RuntimeResult<Value> {
        let ident = interpreter.interpret_tag(self.ident())?;
        if !ident.is_local_identifier() {
            panic!("invalid identifier")
        }
        let ident = ident.take_symbol().ok_or(ErrorKind::InvalidNode)?;
        let value = interpreter.interpret_and_resolve_tag(self.value())?;
        interpreter.define(ident, value)
    }
}

impl Execute for DoNode {
    fn execute(&self, interpreter: &Interpreter) -> RuntimeResult<Value> {
        let mut result = Value::Nil;
        for tag in self.expressions() {
            result = interpreter.interpret_tag(tag)?;
        }
        Ok(result)
    }
}

impl Execute for FunctionCallNode {
    fn execute(&self, interpreter: &Interpreter) -> RuntimeResult<Value> {
        let function = interpreter.interpret_and_resolve_tag(self.function())?;
        match function {
            Value::Function(function) => {
                let parameters = {
                    let node = interpreter.get_node(function.parameters)?;
                    let vector = node.take_vector().ok_or(ErrorKind::InvalidNode)?;
                    let mut parameters = vec![];
                    for tag in vector.items() {
                        parameters.push(interpreter.interpret_tag(tag)?.take_symbol().ok_or(ErrorKind::InvalidNode)?);
                    }
                    parameters.into_iter()
                };
                let arguments = {
                    let mut arguments = vec![];
                    for tag in self.arguments() {
                        arguments.push(interpreter.interpret_and_resolve_tag(tag)?);
                    }
                    arguments.into_iter()
                };

                let mut result = Value::Nil;
                for (s, v) in parameters.zip(arguments) {
                    interpreter.set(s, v);
                }
                // a new scope must be defined after the parameters are set
                interpreter.context.push_scope();
                for b in function.body {
                    result = interpreter.interpret_tag(b)?;
                }
                interpreter.context.pop_scope();
                Ok(result)
            }
            Value::NativeFunction(native) => match native {
                NativeFunction::Now => {
                    let start = SystemTime::now();
                    let since_the_epoch = start
                        .duration_since(UNIX_EPOCH)
                        .expect("time went backwards");
                    Ok(Value::Number(since_the_epoch.as_millis() as f64))
                }
                NativeFunction::Plus => {
                    let mut arguments = self.arguments().into_iter();
                    match arguments.len() {
                        0 => Ok(Value::Number(0.0)),
                        1 => interpreter.interpret_tag(arguments.next().unwrap()),
                        n => {
                            interpreter.context.push_scope();
                            println!("push");
                            interpreter.dump_context();
                            let base = arguments.next().ok_or(ErrorKind::InvalidArgumentArrity)?;
                            let mut base = interpreter.interpret_and_resolve_tag(base)?;
                            for tag in arguments {
                                let value = interpreter.interpret_and_resolve_tag(tag)?;
                                base = interpreter.add(&base, &value)?;
                            }
                            interpreter.context.pop_scope();
                            Ok(base)
                        }
                    }
                }
                NativeFunction::Minus => {
                    let mut arguments = self.arguments().into_iter();
                    match arguments.len() {
                        0 => Ok(Value::Number(0.0)),
                        1 => interpreter.interpret_and_resolve_tag(arguments.next().unwrap()),
                        n => {
                            let base = arguments.next().ok_or(ErrorKind::InvalidArgumentArrity)?;
                            let mut base = interpreter.interpret_and_resolve_tag(base)?;
                            for tag in arguments {
                                let value = interpreter.interpret_and_resolve_tag(tag)?;
                                base = interpreter.sub(&base, &value)?;
                            }
                            Ok(base)
                        }
                    }
                }
                NativeFunction::Multiply => {
                    let mut arguments = self.arguments().into_iter();
                    match arguments.len() {
                        0 => Ok(Value::Number(1.0)),
                        1 => interpreter.interpret_and_resolve_tag(arguments.next().unwrap()),
                        n => {
                            let base = arguments.next().ok_or(ErrorKind::InvalidArgumentArrity)?;
                            let mut base = interpreter.interpret_and_resolve_tag(base)?;
                            for tag in arguments {
                                let value = interpreter.interpret_and_resolve_tag(tag)?;
                                base = interpreter.mul(&base, &value)?;
                            }
                            Ok(base)
                        }
                    }
                }
                NativeFunction::Divide => {
                    let mut arguments = self.arguments().into_iter();
                    match arguments.len() {
                        0 => Ok(Value::Number(1.0)),
                        1 => interpreter.interpret_and_resolve_tag(arguments.next().unwrap()),
                        n => {
                            let base = arguments.next().ok_or(ErrorKind::InvalidArgumentArrity)?;
                            let mut base = interpreter.interpret_and_resolve_tag(base)?;
                            for tag in arguments {
                                let value = interpreter.interpret_and_resolve_tag(tag)?;
                                base = interpreter.div(&base, &value)?;
                            }
                            Ok(base)
                        }
                    }
                }
                NativeFunction::Mod => {
                    let mut arguments = self.arguments().into_iter();
                    let left = arguments
                        .next()
                        .expect("Mod takes 2 arguments. None supplied");
                    let left = interpreter.interpret_and_resolve_tag(left)?;
                    let right = arguments
                        .next()
                        .expect("Mod takes 2 arguments. One supplied");
                    let right = interpreter.interpret_and_resolve_tag(right)?;
                    interpreter.modulus(&left, &right)
                }
                NativeFunction::DumpContext => {
                    interpreter.dump_context();
                    Ok(Value::Nil)
                }
                NativeFunction::Print => {
                    let mut arguments = self.arguments().into_iter();
                    for (i, arg) in arguments.enumerate() {
                        let value = interpreter.interpret_and_resolve_tag(arg)?;
                        if i != 0 {
                            print!(" ");
                        }
                        print!("{}", value);
                    }
                    Ok(Value::Nil)
                }
                NativeFunction::Println => {
                    let mut arguments = self.arguments().into_iter();
                    for (i, arg) in arguments.enumerate() {
                        let value = interpreter.interpret_and_resolve_tag(arg)?;
                        if i != 0 {
                            print!(" ");
                        }
                        print!("{}", value);
                    }
                    print!("\n");
                    Ok(Value::Nil)
                }
                NativeFunction::Eq => {
                    let mut arguments = self.arguments().into_iter();
                    let mut flag = true;
                    let mut last: Option<Value> = None;
                    for arg in arguments {
                        let value = interpreter.interpret_and_resolve_tag(arg)?;
                        if let Some(l) = &last {
                            if interpreter.eq(&l, &value)?.is_truthy() {
                                last = Some(value);
                            } else {
                                flag = false;
                                break;
                            }
                        } else {
                            last = Some(value);
                        }
                    }
                    Ok(Value::Boolean(flag))
                }
                NativeFunction::Or => {
                    let mut arguments = self.arguments().into_iter();
                    for arg in arguments {
                        let value = interpreter.interpret_and_resolve_tag(arg)?;
                        if value.is_truthy() {
                            return Ok(value)
                        }
                    }
                    Ok(Value::Nil)
                }
                NativeFunction::LessThan => {
                    let mut arguments = self.arguments().into_iter();
                    let mut flag = true;
                    let mut last: Option<Value> = None;
                    for arg in arguments {
                        let value = interpreter.interpret_and_resolve_tag(arg)?;
                        if let Some(l) = &last {
                            if interpreter.lt(&l, &value)?.is_truthy() {
                                last = Some(value);
                            } else {
                                flag = false;
                                break;
                            }
                        } else {
                            last = Some(value);
                        }
                    }
                    Ok(Value::Boolean(flag))
                }
                NativeFunction::GreaterThan => {
                    let mut arguments = self.arguments().into_iter();
                    let mut flag = true;
                    let mut last: Option<Value> = None;
                    for arg in arguments {
                        let value = interpreter.interpret_and_resolve_tag(arg)?;
                        if let Some(l) = &last {
                            if interpreter.gt(&l, &value)?.is_truthy() {
                                last = Some(value);
                            } else {
                                flag = false;
                                break;
                            }
                        } else {
                            last = Some(value);
                        }
                    }
                    Ok(Value::Boolean(flag))
                }
            },
            value => panic!("{:?} is not callable", value),
        }
    }
}

impl Execute for FunctionNode {
    fn execute(&self, interpreter: &Interpreter) -> RuntimeResult<Value> {
        Ok(if let Some(name) = self.name() {
            let name = interpreter.interpret_tag(name)?;
            let name = name.take_symbol().ok_or(ErrorKind::NodeNotFound);
            let value = Value::Function(Function {
                parameters: self.parameters(),
                body: self.body(),
            });
            value
        } else {
            Value::Function(Function {
                parameters: self.parameters(),
                body: self.body(),
            })
        })
    }
}
