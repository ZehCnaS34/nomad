use super::Node;
use crate::ast::node::ToNode;
use crate::ast::Tag;
use crate::interpreter::Interpreter;
use crate::result::parser::ErrorKind;

#[derive(Debug, Clone)]
pub struct VectorNode {
    items: Vec<Tag>,
}

impl VectorNode {
    pub fn items(&self) -> Vec<Tag> {
        self.items.iter().map(Clone::clone).collect()
    }
}

impl ToNode for VectorNode {
    fn make_node(tags: Vec<Tag>) -> Result<Node, ErrorKind> {
        Ok(Node::Vector(VectorNode { items: tags }))
    }
}
