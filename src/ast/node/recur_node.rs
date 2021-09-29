use crate::ast::node::{Node, ToNode};
use crate::ast::tag::Partition;
use crate::ast::Tag;
use crate::interpreter::Interpreter;
use crate::result::runtime::ErrorKind;
use crate::result::runtime::ErrorKind::General;
use std::fmt;

#[derive(Debug, Clone)]
pub struct RecurNode {
    pub bindings: Vec<Tag>,
}

impl RecurNode {}

impl ToNode for RecurNode {
    fn make_node(tags: Vec<Node>) -> Result<Node, ErrorKind> {
        // let (_, bindings) = tags.take_1().ok_or(General("Failed"))?;
        todo!()
        // Ok(Node::Recur(RecurNode { bindings }))
    }
}
