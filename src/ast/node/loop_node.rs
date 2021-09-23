use crate::ast::node::{Node, ToNode};
use crate::ast::tag::Partition;
use crate::ast::Tag;
use crate::interpreter::Interpreter;
use crate::result::parser::ErrorKind;
use crate::result::parser::ErrorKind::General;
use std::fmt;

#[derive(Debug, Clone)]
pub struct LoopNode {
    pub bindings: Tag,
    pub body: Vec<Tag>,
}

impl LoopNode {}

impl ToNode for LoopNode {
    fn make_node(tags: Vec<Tag>) -> Result<Node, ErrorKind> {
        let (_, bindings, body) = tags.take_2().ok_or(General("Awesome"))?;
        Ok(Node::Loop(LoopNode { bindings, body }))
    }
}
