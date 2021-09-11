use std::any::Any;
use std::fmt;

use crate::{
    ast,
    ast::{
        node,
        node::{AtomNode, Node, Symbol, ToRational},
        Tag, CHILD_LIMIT,
    },
    copy, interpreter,
    interpreter::{Execute, Interpreter, Introspection, Operation},
    take_tags,
};

trait Show {
    fn show(self) -> Self
    where
        Self: Sized + fmt::Debug;
}

#[derive(Debug, Clone)]
pub struct FunctionCallNode {
    pub function: Tag,
    pub arguments: [Tag; CHILD_LIMIT.function_call],
}

impl FunctionCallNode {
    pub fn from_tags(tags: &[Tag]) -> FunctionCallNode {
        FunctionCallNode {
            function: tags[0],
            arguments: copy! { tags, 1, CHILD_LIMIT.function_call },
        }
    }
}

macro_rules! order_check {
    ($interpreter:ident, $arguments:expr, $operation:expr ) => {
        match crate::ast::Tag::len(&$arguments) {
            0 => crate::ast::node::AtomNode::Boolean(true),
            1 => {
                crate::interpreter::Interpreter::interpret_tag(&$interpreter, $arguments[0]);
                crate::ast::node::AtomNode::Boolean(true)
            }
            n => {
                let mut flag = true;
                let mut last_value =
                    crate::interpreter::Interpreter::interpret_tag(&$interpreter, $arguments[0]);
                for tag in crate::ast::Tag::tags(&$arguments[1..]) {
                    let current =
                        crate::interpreter::Interpreter::interpret_tag(&$interpreter, tag);
                    if $operation(&last_value, &current).unwrap().is_truthy() {
                        last_value = current;
                    } else {
                        flag = false;
                        break;
                    }
                }
                crate::ast::node::atom_node::AtomNode::Boolean(flag)
            }
        }
    };
}

macro_rules! accumulate {
    ($interpreter:ident, $arguments:expr, $operation:expr, $default:expr ) => {
        match crate::ast::Tag::len(&$arguments) {
            0 => $default,
            1 => crate::interpreter::Interpreter::interpret_tag(&$interpreter, $arguments[0]),
            n => {
                let mut sum =
                    crate::interpreter::Interpreter::interpret_tag(&$interpreter, $arguments[0]);
                for tag in crate::ast::Tag::tags(&$arguments[1..]) {
                    let current =
                        crate::interpreter::Interpreter::interpret_tag(&$interpreter, tag);
                    sum = $operation(&sum, &current).unwrap();
                }
                sum
            }
        }
    };
}

impl Execute for FunctionCallNode {
    fn execute(&self, interpreter: &Interpreter, own_tag: Tag) -> AtomNode {
        let function = interpreter.interpret_tag(self.function);
        use AtomNode::Rational;
        match function {
            AtomNode::Symbol(symbol) => {
                if !symbol.is_qualified() {
                    match symbol.name() {
                        "<" => order_check! { interpreter, self.arguments, Operation::lt },
                        ">" => order_check! { interpreter, self.arguments, Operation::gt },
                        "=" => order_check! { interpreter, self.arguments, Operation::eq },
                        "+" => {
                            accumulate! { interpreter, self.arguments, Operation::add, 0.0.to_rational() }
                        }
                        "-" => {
                            accumulate! { interpreter, self.arguments, Operation::sub, 0.0.to_rational() }
                        }
                        "*" => {
                            accumulate! { interpreter, self.arguments, Operation::mul, 1.0.to_rational() }
                        }
                        "/" => {
                            accumulate! { interpreter, self.arguments, Operation::div, 1.0.to_rational() }
                        }
                        "mod" => {
                            let mut arguments: Vec<_> = Tag::tags(&self.arguments).collect();
                            let mut arguments = arguments.into_iter();
                            let left = arguments.next().unwrap();
                            let right = arguments.next().unwrap();
                            let left = interpreter.interpret_tag(left);
                            let right = interpreter.interpret_tag(right);
                            left.imod(&right).unwrap()
                        }
                        "println" => {
                            let l = self.arguments.len() - 1;
                            for (i, tag) in Tag::tags(&self.arguments).enumerate() {
                                print!("{}", interpreter.interpret_tag(tag));
                                if i != l {
                                    print!(" ");
                                }
                            }
                            print!("\n");
                            AtomNode::Nil
                        }
                        function => panic!("fuck {:?}", function),
                    }
                } else {
                    panic!("fuck");
                }
            }
            _ => panic!("fuck"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FunctionNode {
    Named {
        name: Tag,
        parameters: Tag,
        body: [Tag; CHILD_LIMIT.while_body],
    },
    Anonymous {
        parameters: Tag,
        body: [Tag; CHILD_LIMIT.while_body],
    },
}

impl FunctionNode {
    pub fn from_tags(tags: &[Tag]) -> FunctionNode {
        if tags[0].is_atom() {
            FunctionNode::Named {
                name: tags[0],
                parameters: tags[1],
                body: copy! { tags, 2, CHILD_LIMIT.while_body },
            }
        } else if tags[0].is_vector() {
            FunctionNode::Anonymous {
                parameters: tags[0],
                body: copy! { tags, 1, CHILD_LIMIT.while_body },
            }
        } else {
            panic!("Function definitions must start is either a identity or")
        }
    }
}

impl Execute for FunctionNode {
    fn execute(&self, interpreter: &Interpreter, own_tag: Tag) -> AtomNode {
        println!("defining node {:#?}", self);
        AtomNode::Nil
    }
}
