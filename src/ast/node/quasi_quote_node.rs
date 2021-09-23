use crate::ast::Tag;

#[derive(Debug, Clone)]
pub struct QuasiQuoteNode {
    expression: Tag,
}

impl QuasiQuoteNode {
    pub fn from_tag(expression: Tag) -> QuasiQuoteNode {
        QuasiQuoteNode { expression }
    }
}
