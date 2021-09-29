use crate::ast::node::{Node, ToNode};
use crate::ast::tag::Partition;
use crate::ast::Tag;
use crate::defnode;
use crate::interpreter::Interpreter;
use crate::prelude::*;
use crate::result::runtime::ErrorKind;
use crate::result::runtime::ErrorKind::General;

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
