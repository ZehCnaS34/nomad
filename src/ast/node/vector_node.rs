use super::Node;
use crate::ast::node::ToNode;
use crate::ast::Tag;
use crate::defnode;
use crate::interpreter::Interpreter;
use crate::prelude::*;
use crate::result::runtime::ErrorKind;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct VectorNode {
    items: Vec<Node>,
}

defnode! {
    Node::Vector : VectorNode :: nodes => {
        Ok(VectorNode { items: nodes })
    }
}

impl VectorNode {
    pub fn new(items: Vec<Node>) -> VectorNode {
        println!("vecotr.items {:?}", items);
        VectorNode { items }
    }
    pub fn items(&self) -> &Vec<Node> {
        self.items().as_ref()
    }
}
