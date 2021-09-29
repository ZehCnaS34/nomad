use crate::ast::node::{Node, ToNode};
use crate::ast::tag::Partition;
use crate::ast::Tag;
use crate::result::parser::ErrorKind;
use crate::result::parser::ErrorKind::General;

#[derive(Debug, Clone)]
pub struct DecoratorNode {
    pub mutator: Box<Node>,
    pub target: Box<Node>,
}

impl DecoratorNode {}

impl ToNode for DecoratorNode {
    fn make_node(tags: Vec<Node>) -> Result<Node, ErrorKind> {
        let (mutator, target, _) = tags.take_2().ok_or(General("Could not parse"))?;
        Ok(Node::Decorator(DecoratorNode {
            mutator: Box::new(mutator),
            target: Box::new(target),
        }))
    }
}
