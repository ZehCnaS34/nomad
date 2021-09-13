use super::Node;
use crate::ast::Tag;
use crate::ast::CHILD_LIMIT;
use crate::copy;
use crate::interpreter::Interpreter;

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

    pub fn items(&self) -> Vec<Tag> {
        Tag::tags(&self.items[..]).collect()
    }
}
