use crate::ast::node::Node;
use crate::ast::Tag;
use crate::ast::CHILD_LIMIT;
use crate::copy;
use crate::interpreter::Interpreter;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct RecurNode {
    pub bindings: [Tag; CHILD_LIMIT.function_call],
}

impl RecurNode {
    pub fn from_tags(tags: &[Tag]) -> Self {
        RecurNode {
            bindings: copy! { tags, 0, CHILD_LIMIT.function_call },
        }
    }
}
