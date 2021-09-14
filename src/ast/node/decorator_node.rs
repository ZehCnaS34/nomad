use crate::ast::Tag;

#[derive(Debug, Clone)]
pub struct DecoratorNode {
    mutator: Tag,
    target: Tag,
}

impl DecoratorNode {
    pub fn from_tags(mutator: Tag, target: Tag) -> DecoratorNode {
        DecoratorNode { mutator, target }
    }
}
