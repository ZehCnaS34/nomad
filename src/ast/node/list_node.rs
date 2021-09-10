use super::Node;
use crate::ast::parser::Tag;
use crate::ast::CHILD_LIMIT;
use crate::copy;
use crate::interpreter::{Execute, Interpreter};

#[derive(Debug, Clone)]
pub struct ListNode {
    items: [Tag; CHILD_LIMIT.program],
}

impl ListNode {
    pub fn from_tags(tags: &[Tag]) -> Self {
        ListNode {
            items: copy! { tags, 0, CHILD_LIMIT.program },
        }
    }
}

impl Execute for ListNode {
    fn execute(&self, interpreter: &Interpreter, own_tag: Tag) {
        todo!();
    }
}
