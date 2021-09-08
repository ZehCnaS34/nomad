use crate::ast::node::atom_node::Symbol;
use crate::ast::node::Node;
use crate::runtime::{Execution, Runtime};
use crate::value::{NValue, Function};
use std::fmt;

#[derive(Debug, Clone)]
pub struct FunctionCallNode {
    pub function: Box<Node>,
    pub arguments: Vec<Node>,
}

impl Execution for FunctionCallNode {
    fn execute(&self, runtime: &mut Runtime) -> NValue {
        let value = runtime.execute(self.function.as_ref());
        match value {
            NValue::NFunction(function) => match function {
                Function::Println => {
                    for node in &self.arguments {
                        let result = runtime.execute(node);
                        print!("{}", result);
                    }
                    print!("\n");
                    NValue::Nil
                }
                Function::Multiplication => {
                    let mut values = vec![];
                    for node in &self.arguments {
                        values.push(runtime.execute(node));
                    }
                    println!("mutliplication");
                    NValue::Nil
                }
                Function::Addition => {
                    let mut values = vec![];
                    for node in &self.arguments {
                        values.push(runtime.execute(node));
                    }
                    let mut sum = NValue::NNumber(Box::new(0.0));
                    let mut values = values.into_iter();
                    for current in values {
                        sum = NValue::add(&sum, &current);
                    }
                    sum
                }
                Function::Division => {
                    let mut values = vec![];
                    for node in &self.arguments {
                        values.push(runtime.execute(node));
                    }
                    NValue::Nil
                }
                Function::Subtraction => {
                    let mut values = vec![];
                    for node in &self.arguments {
                        values.push(runtime.execute(node));
                    }
                    NValue::Nil
                }
                Function::LessThan => {
                    let mut values = vec![];
                    for node in &self.arguments {
                        values.push(runtime.execute(node));
                    }
                    let mut flag = true;
                    let mut values = values.into_iter();
                    let mut previous = values.next().unwrap();
                    for current in values {
                        let result = NValue::less_than(&previous, &current);
                        if !result.is_truthy() {
                            flag = false;
                            break;
                        }
                        previous = current;
                    }
                    NValue::NBoolean(flag)
                }
                Function::GreaterThan => {
                    let mut values = vec![];
                    for node in &self.arguments {
                        values.push(runtime.execute(node));
                    }
                    NValue::Nil
                }
                Function::Runtime{parameters, body} => {
                    NValue::Nil
                }
                _ => NValue::Nil
            }
            _ => NValue::Nil
        }
    }
}

impl fmt::Display for FunctionCallNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}", self.function)?;
        for (i, argument) in self.arguments.iter().enumerate() {
            if i == 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", argument)?;
            if i + 1 < self.arguments.len() {
                write!(f, " ")?;
            }
        }
        write!(f, ")")?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum FunctionNode {
    Anonymous {
        parameters: Vec<Symbol>,
        body: Vec<Node>,
    },
}

impl Execution for FunctionNode {
    fn execute(&self, runtime: &mut Runtime) -> NValue {
        NValue::Nil
    }
}
