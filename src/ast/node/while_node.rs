use crate::{
    ast::{
        node,
        node::{AtomNode, Node},
        Tag, CHILD_LIMIT,
    },
    copy,
    interpreter::{Execute, Interpreter, Introspection},
};
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct WhileNode {
    pub condition: Tag,
    pub body: [Tag; CHILD_LIMIT.while_body],
}

impl WhileNode {
    pub fn from_tags(tags: &[Tag]) -> Self {
        let condition = tags[0];
        let body = copy! { tags, 1, CHILD_LIMIT.while_body };
        WhileNode { condition, body }
    }
}

impl Execute for WhileNode {
    fn execute(&self, interpreter: &Interpreter, own_tag: Tag) -> AtomNode {
        loop {
            if !interpreter.interpret_tag(self.condition).is_truthy() {
                break;
            }
            for tag in Tag::tags(&self.body) {
                interpreter.interpret_tag(tag);
            }
        }

        AtomNode::Nil
    }
}
