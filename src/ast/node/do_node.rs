use crate::ast::node::{Node, ToNode};
use crate::ast::tag::Partition;
use crate::ast::Tag;
use crate::interpreter::Interpreter;
use crate::result::parser::ErrorKind;
use crate::result::parser::ErrorKind::General;

#[derive(Debug, Clone)]
pub struct DoNode {
    expressions: Vec<Node>,
}

impl DoNode {
    pub fn expressions(&self) -> &Vec<Node> {
        self.expressions.as_ref()
    }
}

impl ToNode for DoNode {
    fn make_node(tags: Vec<Node>) -> Result<Node, ErrorKind> {
        todo!()
    }
}
