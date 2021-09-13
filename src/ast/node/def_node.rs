use crate::ast::node::Node;
use crate::ast::Tag;
use crate::ast::CHILD_LIMIT;
use crate::interpreter::{Interpreter};
use std::fmt;
use std::vec::IntoIter;

#[derive(Debug, Copy, Clone)]
pub struct DefinitionNode {
    ident: Tag,
    value: Tag,
}

impl DefinitionNode {
    pub fn from_tags(tags: &[Tag]) -> DefinitionNode {
        let ident = tags[0];
        let value = tags[1];
        DefinitionNode { ident, value }
    }

    pub fn ident(&self) -> Tag {
        self.ident
    }

    pub fn value(&self) -> Tag {
        self.value
    }
}
