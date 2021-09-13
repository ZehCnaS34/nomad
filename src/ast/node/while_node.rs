use crate::{
    ast::{node, node::Node, Tag, TagIter, CHILD_LIMIT},
    copy,
    interpreter::{Interpreter, Introspection},
};
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct WhileNode {
    condition: Tag,
    body: [Tag; CHILD_LIMIT.while_body],
}

impl WhileNode {
    pub fn from_tags(tags: &[Tag]) -> Self {
        let condition = tags[0];
        let body = copy! { tags, 1, CHILD_LIMIT.while_body };
        WhileNode { condition, body }
    }

    pub fn condition(&self) -> Tag {
        self.condition
    }

    pub fn body(&self) -> TagIter {
        Tag::tags(&self.body[..])
    }
}
