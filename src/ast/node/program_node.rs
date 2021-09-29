use crate::ast::node::{Node, ToNode};
use crate::ast::Tag;
use crate::interpreter::Interpreter;
use crate::result::runtime::ErrorKind;

#[derive(Debug, Clone)]
pub struct ProgramNode {
    expressions: Vec<Node>,
}

impl ProgramNode {
    pub fn new(expressions: Vec<Node>) -> ProgramNode {
        ProgramNode { expressions }
    }
    pub fn expressions(&self) -> &Vec<Node> {
        &self.expressions
    }
}
