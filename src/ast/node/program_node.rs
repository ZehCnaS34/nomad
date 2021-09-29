use crate::ast::node::Node;

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
