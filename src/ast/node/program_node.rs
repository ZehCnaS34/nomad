use crate::ast::node::{Node, ToNode};
use crate::ast::Tag;
use crate::interpreter::Interpreter;
use crate::result::parser::ErrorKind;

#[derive(Debug, Clone)]
pub struct ProgramNode {
    expressions: Vec<Tag>,
}

impl ProgramNode {
    pub fn expressions(&self) -> Vec<Tag> {
        self.expressions.iter().map(Clone::clone).collect()
    }
}

impl ToNode for ProgramNode {
    fn make_node(tags: Vec<Tag>) -> Result<Node, ErrorKind> {
        Ok(Node::Program(ProgramNode { expressions: tags }))
    }
}
