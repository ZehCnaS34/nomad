use crate::ast::Tag;
use crate::ast::CHILD_LIMIT;
use crate::copy;
use crate::interpreter::{Execute, Interpreter};

#[derive(Debug, Copy, Clone)]
pub struct DoNode {
    expressions: [Tag; CHILD_LIMIT.while_body],
}

impl DoNode {
    pub fn from_tags(tags: &[Tag]) -> DoNode {
        DoNode {
            expressions: copy! { tags, 0, CHILD_LIMIT.while_body },
        }
    }
}
