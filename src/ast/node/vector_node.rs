use super::Node;
use crate::ast::node::AtomNode;
use crate::ast::Tag;
use crate::ast::CHILD_LIMIT;
use crate::copy;
use crate::interpreter::{Execute, Interpreter};

#[derive(Debug, Clone)]
pub struct VectorNode {
    items: [Tag; CHILD_LIMIT.program],
}

impl VectorNode {
    pub fn from_tags(tags: &[Tag]) -> Self {
        VectorNode {
            items: copy! { tags, 0, CHILD_LIMIT.program },
        }
    }
}

impl Execute for VectorNode {
    fn execute(&self, interpreter: &Interpreter, own_tag: Tag) -> AtomNode {
        todo!();
    }
}