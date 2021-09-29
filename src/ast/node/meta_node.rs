use crate::ast::node::{Node, ToNode};
use crate::ast::tag::Partition;
use crate::ast::Tag;
use crate::result::parser::ErrorKind;
use crate::result::parser::ErrorKind::General;

#[derive(Debug, Clone)]
pub struct MetaNode {
    data: Tag,
    target: Tag,
}

impl MetaNode {}

impl ToNode for MetaNode {
    fn make_node(tags: Vec<Node>) -> Result<Node, ErrorKind> {
        todo!()
        // let (data, target, _) = tags.take_2().ok_or(General("Failed"))?;
        // Ok(Node::Meta(MetaNode { data, target }))
    }
}
