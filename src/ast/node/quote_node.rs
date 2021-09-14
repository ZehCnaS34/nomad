use crate::ast::Tag;

#[derive(Clone, Debug)]
pub struct QuoteNode {
    expression: Tag,
}

impl QuoteNode {
    pub fn from_tag(tag: Tag) -> QuoteNode {
        QuoteNode { expression: tag }
    }

    pub fn expression(&self) -> Tag {
        self.expression
    }
}
