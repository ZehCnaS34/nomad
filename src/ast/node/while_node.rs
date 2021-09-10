use crate::ast::node::Node;
use crate::ast::parser::Tag;
use crate::ast::CHILD_LIMIT;
use crate::copy;
use crate::interpreter::{Execute, Interpreter};
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
    fn execute(&self, interpreter: &Interpreter, own_tag: Tag) {
        let mut i = 0;
        loop {
            i += 1;
            interpreter.interpret_tag(self.condition);
            if !interpreter.is_tag_true(self.condition) || i > 30 {
                break;
            }
            for tag in Tag::tags(&self.body) {
                interpreter.interpret_tag(tag);
            }
        }
    }
}
