use super::Node;
use crate::ast::node::ToNode;
use crate::ast::Tag;
use crate::interpreter::Interpreter;
use crate::result::parser::ErrorKind;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct VectorNode {
    items: Vec<Node>,
}

impl TryFrom<Vec<Node>> for VectorNode {
    type Error = ErrorKind;
    fn try_from(items: Vec<Node>) -> Result<Self, Self::Error> {
        Ok(VectorNode { items })
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
