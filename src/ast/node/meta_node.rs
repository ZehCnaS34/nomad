use crate::ast::{Tag, TagIter};

#[derive(Debug, Clone)]
pub struct MetaNode {
    data: Tag,
    target: Tag,
}

impl MetaNode {
    pub fn from_tags(data: Tag, target: Tag) -> MetaNode {
        MetaNode { data, target }
    }
}
