use crate::ast::node::{Node, ToNode};
use crate::ast::tag::Partition;
use crate::ast::Tag;
use crate::interpreter::Interpreter;
use crate::result::parser::ErrorKind;
use std::fmt;
use std::vec::IntoIter;

#[derive(Debug, Copy, Clone)]
pub struct DefinitionNode {
    ident: Tag,
    value: Tag,
}

impl DefinitionNode {
    pub fn ident(&self) -> Tag {
        self.ident
    }

    pub fn value(&self) -> Tag {
        self.value
    }
}

impl ToNode for DefinitionNode {
    fn make_node(tags: Vec<Tag>) -> Result<Node, ErrorKind> {
        let (_, ident, value, _) = tags.take_3().ok_or(ErrorKind::General("Failed"))?;
        let def = DefinitionNode { ident, value };
        Ok(Node::Definition(def))
    }
}
