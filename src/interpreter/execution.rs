use super::operation::*;
use super::value::*;
use super::Interpreter;
use super::NodeQuery;
use crate::ast::node::*;
use crate::ast::tag::Partition;

use crate::result::runtime::ErrorKind;
use crate::result::Result;
use std::time::{SystemTime, UNIX_EPOCH};

pub trait Execute {
    fn execute(&self, interpreter: &Interpreter) -> Result<Value>;
}

impl Execute for Node {
    fn execute(&self, interpreter: &Interpreter) -> Result<Value> {
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
    fn execute(&self, interpreter: &Interpreter) -> Result<Value> {
        Ok(Value::Symbol(Symbol::from_node(self.clone())))
    }
}

impl Execute for QuasiQuoteNode {
    fn execute(&self, interpreter: &Interpreter) -> Result<Value> {
        println!("{:#?}", self);
        todo!()
    }
}

impl Execute for MacroNode {
    fn execute(&self, interpreter: &Interpreter) -> Result<Value> {
        println!("{:#?}", self);
        todo!()
    }
}

impl Execute for DecoratorNode {
    fn execute(&self, interpreter: &Interpreter) -> Result<Value> {
        todo!("decoration nodes are not defined");
    }
}

impl Execute for VectorNode {
    fn execute(&self, interpreter: &Interpreter) -> Result<Value> {
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
    fn execute(&self, interpreter: &Interpreter) -> Result<Value> {
        let mut result = Value::Nil;
        for tag in self.expressions() {
            result = tag.execute(interpreter)?;
        }
        Ok(result)
    }
}

impl Execute for IfNode {
    fn execute(&self, interpreter: &Interpreter) -> Result<Value> {
        if self.condition.execute(interpreter)?.truthy() {
            self.true_branch.execute(interpreter)
        } else {
            self.false_branch.execute(interpreter)
        }
    }
}

impl Execute for QuoteNode {
    fn execute(&self, interpreter: &Interpreter) -> Result<Value> {
        interpreter.interpret_tag(self.expression())
    }
}

impl Execute for MetaNode {
    fn execute(&self, interpreter: &Interpreter) -> Result<Value> {
        todo!("meta data node not defined");
    }
}

impl Execute for WhileNode {
    fn execute(&self, interpreter: &Interpreter) -> Result<Value> {
        loop {
            let condition = self.condition().execute(interpreter)?;
            if !condition.truthy() {
                break;
            }
            for tag in self.body() {
                tag.execute(interpreter)?;
            }
        }
        Ok(Value::Nil)
    }
}

impl Execute for DefinitionNode {
    fn execute(&self, interpreter: &Interpreter) -> Result<Value> {
        let ident = self.ident().execute(interpreter)?;
        if !ident.is_local_identifier() {
            panic!("invalid identifier")
        }
        let ident = ident.take_symbol().ok_or(ErrorKind::InvalidNode)?;
        let value = self.value().execute(interpreter)?;
        interpreter.define(ident, value)
    }
}

impl Execute for DoNode {
    fn execute(&self, interpreter: &Interpreter) -> Result<Value> {
        let mut result = Value::Nil;
        for tag in self.expressions() {
            result = tag.execute(interpreter)?;
        }
        Ok(result)
    }
}

impl Execute for FunctionCallNode {
    fn execute(&self, interpreter: &Interpreter) -> Result<Value> {
        let function = interpreter.interpret_and_resolve_tag(self.function())?;
        match function {
            Value::Function(function) => {
                todo!()
                // let parameters = function.parameters.into_iter();
                // let arguments = {
                //     let mut arguments = vec![];
                //     for tag in self.arguments() {
                //         arguments.push(interpreter.interpret_and_resolve_tag(tag)?);
                //     }
                //     arguments.into_iter()
                // };
                //
                // let mut result = Value::Nil;
                // for (s, v) in parameters.zip(arguments) {
                //     interpreter.set(s, v);
                // }
                // // a new scope must be defined after the parameters are set
                // interpreter.context.push_scope();
                // for b in function.body {
                //     result = interpreter.interpret_tag(b)?;
                // }
                // interpreter.context.pop_scope();
                // Ok(result)
            }
            value => panic!("{:?} is not callable", value),
        }
    }
}

// fn all_symbols(parameters: &VectorNode) -> bool {
//     parameters.items().iter().all(|item| item.is_symbol())
// }

impl Execute for FunctionNode {
    fn execute(&self, interpreter: &Interpreter) -> Result<Value> {
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
