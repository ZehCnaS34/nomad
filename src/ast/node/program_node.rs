use crate::ast::CHILD_LIMIT;
use crate::ast::{Tag, TagIter};
use crate::copy;
use crate::interpreter::Interpreter;

#[derive(Debug, Copy, Clone)]
pub struct ProgramNode {
    expressions: [Tag; CHILD_LIMIT.program],
}

impl ProgramNode {
    pub fn from(tags: &[Tag]) -> ProgramNode {
        ProgramNode {
            expressions: copy! { tags, 0, CHILD_LIMIT.program },
        }
    }

    pub fn expressions(&self) -> TagIter {
        Tag::tags(&self.expressions[..])
    }
}
