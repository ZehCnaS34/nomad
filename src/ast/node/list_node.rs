use super::Node;
use crate::ast::node::Node::List;
use crate::ast::node::ToNode;
use crate::result::runtime::ErrorKind;

#[derive(Debug, Clone)]
pub struct ListNode {
    items: Vec<Node>,
}

impl ListNode {}

impl ToNode for ListNode {
    fn make_node(tags: Vec<Node>) -> Result<Node, ErrorKind> {
        Ok(List(ListNode { items: tags }))
    }
}
