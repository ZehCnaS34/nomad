use crate::ast::node::{Node, ToNode, VectorNode};
use crate::ast::tag::Partition;
use crate::ast::Tag;
use crate::interpreter::Interpreter;
use crate::result::parser::ErrorKind;
use crate::result::parser::ErrorKind::General;
use std::fmt;

#[derive(Debug, Clone)]
pub struct LoopNode {
    pub bindings: VectorNode,
    pub body: Vec<Node>,
}

impl LoopNode {}

impl ToNode for LoopNode {
    fn make_node(tags: Vec<Node>) -> Result<Node, ErrorKind> {
        todo!()        
    }
}
