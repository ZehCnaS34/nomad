use crate::ast::node::Node;
use crate::ast::Tag;
use crate::ast::CHILD_LIMIT;
use crate::copy;
use crate::interpreter::{Execute, Interpreter};
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct LoopNode {
    pub bindings: Tag,
    pub body: [Tag; CHILD_LIMIT.while_body],
}

impl LoopNode {
    pub fn from_tags(tags: &[Tag]) -> Self {
        LoopNode {
            bindings: tags[0],
            body: copy! { tags, 1, CHILD_LIMIT.while_body },
        }
    }
}
