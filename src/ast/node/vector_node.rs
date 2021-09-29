use super::Node;
use crate::prelude::*;
use crate::defnode;
use crate::ast::node::ToNode;
use crate::ast::Tag;
use crate::interpreter::Interpreter;
use crate::result::parser::ErrorKind;
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
