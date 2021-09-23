use super::Node;
use crate::ast::node::Node::List;
use crate::ast::node::ToNode;
use crate::ast::Tag;
use crate::interpreter::Interpreter;
use crate::result::parser::ErrorKind;

#[derive(Debug, Clone)]
pub struct ListNode {
    items: Vec<Tag>,
}

impl ListNode {}

impl ToNode for ListNode {
    fn make_node(tags: Vec<Tag>) -> Result<Node, ErrorKind> {
        Ok(List(ListNode { items: tags }))
    }
}
