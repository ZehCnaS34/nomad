use crate::ast::TagIter;
use crate::{
    ast,
    ast::{Tag, CHILD_LIMIT},
    copy, take_tags,
};

#[derive(Debug, Clone)]
pub struct LetNode {
    bindings: Tag,
    body: Vec<Tag>,
}

impl LetNode {
    pub fn from_tags(tags: &[Tag]) -> LetNode {
        LetNode {
            bindings: tags[1],
            body: Tag::tags(&tags[..]).collect(),
        }
    }
}
