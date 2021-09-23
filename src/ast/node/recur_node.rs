use crate::ast::node::{Node, ToNode};
use crate::ast::tag::Partition;
use crate::ast::Tag;
use crate::interpreter::Interpreter;
use crate::result::parser::ErrorKind;
use crate::result::parser::ErrorKind::General;
use std::fmt;

#[derive(Debug, Clone)]
pub struct RecurNode {
    pub bindings: Vec<Tag>,
}

impl RecurNode {}

impl ToNode for RecurNode {
    fn make_node(tags: Vec<Tag>) -> Result<Node, ErrorKind> {
        let (_, bindings) = tags.take_1().ok_or(General("Failed"))?;
        Ok(Node::Recur(RecurNode { bindings }))
    }
}
