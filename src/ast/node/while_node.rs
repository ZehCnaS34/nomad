use crate::defnode;
use crate::prelude::*;
use crate::result::runtime;
use crate::{
    ast::{node, node::Node, node::ToNode, tag::Partition, Tag},
    interpreter::Interpreter,
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct WhileNode {
    condition: Box<Node>,
    body: Vec<Node>,
}

defnode! {
    Node::While : WhileNode :: nodes => {
        let (_, condition, body) = nodes.take_2().ok_or(CouldNotParseAtom)?;
        Ok(WhileNode { condition: Box::new(condition), body })
    }
}

impl WhileNode {
    pub fn condition(&self) -> &Node {
        self.condition.as_ref()
    }

    pub fn body(&self) -> &Vec<Node> {
        &self.body
    }
}
