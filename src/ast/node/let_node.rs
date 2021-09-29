use crate::ast::node::{Node, ToNode};
use crate::ast::tag::Partition;
use crate::result::runtime::ErrorKind;
use crate::result::runtime::ErrorKind::General;
use crate::{ast, ast::Tag};

#[derive(Debug, Clone)]
pub struct LetNode {
    bindings: Box<Node>,
    body: Vec<Node>,
}

impl LetNode {}

impl ToNode for LetNode {
    fn make_node(tags: Vec<Node>) -> Result<Node, ErrorKind> {
        todo!()
        // let (_form, bindings, body) = tags.take_2().ok_or(General("Failed"))?;
        // Ok(Node::Let(LetNode {
        //     bindings: Box::new(bindings),
        //     body: Vec::new(body),
        // }))
    }
}
