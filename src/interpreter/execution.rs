use super::operation::*;
use super::value::*;
use super::Interpreter;
use super::NodeQuery;
use crate::ast::node::*;
use crate::ast::tag::Partition;
use crate::interpreter::value::NativeFunction;
use crate::result::runtime::ErrorKind;
use crate::result::RuntimeResult;
use std::time::{SystemTime, UNIX_EPOCH};

pub trait Execute {
    fn execute(&self, interpreter: &Interpreter) -> RuntimeResult<Value>;
}

impl Execute for Node {
    fn execute(&self, interpreter: &Interpreter) -> RuntimeResult<Value> {
        match self {
            Node::Boolean(node) => Ok(Value::make_bool(node.value())),
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
            Node::Number(number) => Ok(Value::make_number(number.value())),
            Node::Program(node) => node.execute(interpreter),
            Node::Quote(node) => node.execute(interpreter),
            Node::Recur(node) => todo!(),
            Node::String(node) => Ok(Value::String(String {
                value: node.value().to_string(),
            })),
            Node::Symbol(node) => node.execute(interpreter),
            Node::Vector(node) => node.execute(interpreter),
            Node::While(node) => node.execute(interpreter),
            Node::Macro(node) => node.execute(interpreter),
            Node::QuasiQuote(node) => node.execute(interpreter),
        }
    }
}

impl Execute for SymbolNode {
    fn execute(&self, interpreter: &Interpreter) -> RuntimeResult<Value> {
        Ok(Value::Symbol(Symbol::from_node(self.clone())))
    }
}

impl Execute for QuasiQuoteNode {
    fn execute(&self, interpreter: &Interpreter) -> RuntimeResult<Value> {
        println!("{:#?}", self);
        todo!()
    }
}

impl Execute for MacroNode {
    fn execute(&self, interpreter: &Interpreter) -> RuntimeResult<Value> {
        println!("{:#?}", self);
        todo!()
    }
}

impl Execute for DecoratorNode {
    fn execute(&self, interpreter: &Interpreter) -> RuntimeResult<Value> {
        todo!("decoration nodes are not defined");
    }
}

impl Execute for VectorNode {
    fn execute(&self, interpreter: &Interpreter) -> RuntimeResult<Value> {
        let mut vector = Vector::new();
        for tag in self.items() {
            let value = tag.execute(interpreter)?;
            todo!("resolve symbols in vector");
            vector = vector.push(value);
        }
        Ok(Value::Vector(vector))
    }
}

impl Execute for ProgramNode {
    fn execute(&self, interpreter: &Interpreter) -> RuntimeResult<Value> {
        let mut result = Value::Nil;
        for tag in self.expressions() {
            result = tag.execute(interpreter)?;
        }
        Ok(result)
    }
}

impl Execute for IfNode {
    fn execute(&self, interpreter: &Interpreter) -> RuntimeResult<Value> {
        if self.condition.execute(interpreter)?.truthy()
        {
            self.true_branch.execute(interpreter)
        } else {
            self.false_branch.execute(interpreter)
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
            if !condition.truthy() {
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
            result = tag.execute(interpreter)?;
        }
        Ok(result)
    }
}

impl Execute for FunctionCallNode {
    fn execute(&self, interpreter: &Interpreter) -> RuntimeResult<Value> {
        let function = interpreter.interpret_and_resolve_tag(self.function())?;
        match function {
            Value::Function(function) => {
                let parameters = function.parameters.into_iter();
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
                NativeFunction::Count => {
                    let (object, rest) = interpreter
                        .interpret_and_resolve_tags(self.arguments())?
                        .take_1()
                        .ok_or(ErrorKind::InvalidOperation)?;
                    Ok(Value::Number(Number::from(match object {
                        Value::String(string) => string.length(),
                        Value::Vector(vector) => vector.length(),
                        _ => return Err(ErrorKind::InvalidOperation),
                    })))
                }
                NativeFunction::Conj => {
                    let arguments: Vec<_> = self
                        .arguments()
                        .iter()
                        .flat_map(|tag| interpreter.interpret_and_resolve_tag(*tag))
                        .collect();
                    let (object, value, rest) =
                        arguments.take_2().ok_or(ErrorKind::InvalidOperation)?;
                    object.conj(value)
                }
                NativeFunction::Get => {
                    let arguments: Vec<_> = self
                        .arguments()
                        .iter()
                        .flat_map(|tag| interpreter.interpret_and_resolve_tag(*tag))
                        .collect();
                    let (object, key, rest) =
                        arguments.take_2().ok_or(ErrorKind::InvalidOperation)?;
                    object.lookup(key).map(|value| value.clone())
                }
                NativeFunction::Now => {
                    let start = SystemTime::now();
                    let since_the_epoch = start
                        .duration_since(UNIX_EPOCH)
                        .expect("time went backwards");
                    Ok(Value::Number(since_the_epoch.as_millis().into()))
                }
                NativeFunction::Plus => {
                    let mut arguments = self.arguments().into_iter();
                    match arguments.len() {
                        0 => Ok(Value::Number(0.0.into())),
                        1 => interpreter.interpret_tag(arguments.next().unwrap()),
                        n => {
                            interpreter.context.push_scope();
                            let base = arguments.next().ok_or(ErrorKind::InvalidArgumentArity)?;
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
                        0 => Ok(Value::Number(0.0.into())),
                        1 => interpreter.interpret_and_resolve_tag(arguments.next().unwrap()),
                        n => {
                            let base = arguments.next().ok_or(ErrorKind::InvalidArgumentArity)?;
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
                        0 => Ok(Value::Number(1.0.into())),
                        1 => interpreter.interpret_and_resolve_tag(arguments.next().unwrap()),
                        n => {
                            let base = arguments.next().ok_or(ErrorKind::InvalidArgumentArity)?;
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
                        0 => Ok(Value::Number(1.0.into())),
                        1 => interpreter.interpret_and_resolve_tag(arguments.next().unwrap()),
                        n => {
                            let base = arguments.next().ok_or(ErrorKind::InvalidArgumentArity)?;
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
                            if interpreter.eq(&l, &value)?.truthy() {
                                last = Some(value);
                            } else {
                                flag = false;
                                break;
                            }
                        } else {
                            last = Some(value);
                        }
                    }
                    Ok(Value::Boolean(flag.into()))
                }
                NativeFunction::Or => {
                    let mut arguments = self.arguments().into_iter();
                    for arg in arguments {
                        let value = interpreter.interpret_and_resolve_tag(arg)?;
                        if value.truthy() {
                            return Ok(value);
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
                            if interpreter.lt(&l, &value)?.truthy() {
                                last = Some(value);
                            } else {
                                flag = false;
                                break;
                            }
                        } else {
                            last = Some(value);
                        }
                    }
                    Ok(Value::Boolean(flag.into()))
                }
                NativeFunction::GreaterThan => {
                    let arguments = self.arguments().into_iter();
                    let mut flag = true;
                    let mut last: Option<Value> = None;
                    for arg in arguments {
                        let value = interpreter.interpret_and_resolve_tag(arg)?;
                        if let Some(l) = &last {
                            if interpreter.gt(&l, &value)?.truthy() {
                                last = Some(value);
                            } else {
                                flag = false;
                                break;
                            }
                        } else {
                            last = Some(value);
                        }
                    }
                    Ok(Value::Boolean(flag.into()))
                }
            },
            value => panic!("{:?} is not callable", value),
        }
    }
}

// fn all_symbols(parameters: &VectorNode) -> bool {
//     parameters.items().iter().all(|item| item.is_symbol())
// }

impl Execute for FunctionNode {
    fn execute(&self, interpreter: &Interpreter) -> RuntimeResult<Value> {
        todo!()
        // let parameter_nodes = interpreter.get_vector(self.parameters())?;
        // if all_symbols(&parameter_nodes) {
        //     let parameters: Vec<_> = interpreter
        //         .get_symbols(&parameter_nodes.items())?
        //         .iter()
        //         .flat_map(|node| {
        //             node.execute(interpreter)
        //                 .ok()
        //                 .and_then(|value| value.take_symbol())
        //         })
        //         .collect();
        //     Ok(Value::Function(Function {
        //         name: {
        //             self.name()
        //                 .and_then(|name| interpreter.get_symbol(name).ok())
        //                 .and_then(|symbol_node| symbol_node.execute(interpreter).ok())
        //                 .and_then(|value| value.take_symbol())
        //         },
        //         parameters,
        //         body: self.body(),
        //     }))
        // } else {
        //     todo!("handle destructing and variadic parameters")
        // }
    }
}
