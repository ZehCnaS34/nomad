use crate::ast::node::{Node, ToNode, VectorNode};
use crate::ast::tag::Partition;
use crate::defnode;
use crate::prelude::*;
use crate::result::runtime::ErrorKind;
use crate::result::runtime::ErrorKind::General;

#[derive(Debug, Clone)]
pub struct LetNode {
    bindings: VectorNode,
    body: Vec<Node>,
}

impl LetNode {}

defnode! {
    Node::Let : LetNode :: nodes => {
        let (_, bindings, body) = nodes.take_2().ok_or(General("invalid let node"))?;
        let bindings = bindings.take_vector().ok_or(General("let bindings must be a vector"))?;
        Ok(LetNode{
            bindings,
            body
        })
    }
}
