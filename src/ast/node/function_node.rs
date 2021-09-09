use crate::ast::node::atom_node::Symbol;
use crate::ast::node::Node;
use std::fmt;

#[derive(Debug, Clone)]
pub struct FunctionCallNode {
    pub function: Box<Node>,
    pub arguments: Vec<Node>,
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

