use crate::ast::node::{Node, ToNode};
use crate::ast::tag::Partition;
use crate::ast::TagIter;
use crate::result::parser::ErrorKind;
use crate::result::parser::ErrorKind::General;
use crate::{ast, ast::Tag};

#[derive(Debug, Clone)]
pub struct LetNode {
    bindings: Tag,
    body: Vec<Tag>,
}

impl LetNode {}

impl ToNode for LetNode {
    fn make_node(tags: Vec<Tag>) -> Result<Node, ErrorKind> {
        let (_form, bindings, body) = tags.take_2().ok_or(General("Failed"))?;
        Ok(Node::Let(LetNode { bindings, body }))
    }
}
