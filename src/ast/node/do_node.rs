use crate::ast::node::{Node, ToNode};
use crate::ast::tag::Partition;
use crate::ast::Tag;
use crate::interpreter::Interpreter;
use crate::result::parser::ErrorKind;
use crate::result::parser::ErrorKind::General;

#[derive(Debug, Clone)]
pub struct DoNode {
    expressions: Vec<Tag>,
}

impl DoNode {
    pub fn expressions(&self) -> Vec<Tag> {
        self.expressions.iter().map(Clone::clone).collect()
    }
}

impl ToNode for DoNode {
    fn make_node(tags: Vec<Tag>) -> Result<Node, ErrorKind> {
        let (_, expressions) = tags.take_1().ok_or(General("Failed"))?;
        Ok(Node::Do(DoNode { expressions }))
    }
}
