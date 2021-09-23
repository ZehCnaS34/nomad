use crate::result::parser;
use crate::{
    ast::{node, node::Node, node::ToNode, tag::Partition, Tag, TagIter},
    interpreter::Interpreter,
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct WhileNode {
    condition: Tag,
    body: Vec<Tag>,
}

impl ToNode for WhileNode {
    fn make_node(tags: Vec<Tag>) -> Result<Node, parser::ErrorKind> {
        let (_, condition, body) = tags.take_2().ok_or(parser::ErrorKind::CouldNotParseAtom)?;
        Ok(Node::While(WhileNode { condition, body }))
    }
}

impl WhileNode {
    pub fn condition(&self) -> Tag {
        self.condition
    }

    pub fn body(&self) -> TagIter {
        Tag::tags(&self.body[..])
    }
}
