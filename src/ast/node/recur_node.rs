use crate::ast::node::{Node, ToNode};
use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct RecurNode {
    pub bindings: Vec<Tag>,
}

impl RecurNode {}

impl ToNode for RecurNode {
    fn make_node(_: Vec<Node>) -> Result<Node> {
        // let (_, bindings) = tags.take_1().ok_or(General("Failed"))?;
        todo!()
        // Ok(Node::Recur(RecurNode { bindings }))
    }
}
