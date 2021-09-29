use crate::ast::node::{Node, ToNode, VectorNode};
use crate::result::runtime::ErrorKind;

#[derive(Debug, Clone)]
pub struct LoopNode {
    pub bindings: VectorNode,
    pub body: Vec<Node>,
}

impl LoopNode {}

impl ToNode for LoopNode {
    fn make_node(_: Vec<Node>) -> Result<Node, ErrorKind> {
        todo!()
    }
}
