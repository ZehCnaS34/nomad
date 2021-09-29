use crate::ast::node::{Node, ToNode};
use crate::defnode;
use crate::prelude::*;
use crate::result::runtime::ErrorKind;

#[derive(Debug, Clone)]
pub struct DoNode {
    expressions: Vec<Node>,
}

impl DoNode {
    pub fn expressions(&self) -> &Vec<Node> {
        self.expressions.as_ref()
    }
}

defnode! {
    Node::Do : DoNode :: nodes => {
        Ok(DoNode { expressions: nodes })
    }
}
