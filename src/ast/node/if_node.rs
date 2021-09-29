use crate::ast::node::Node;
use crate::ast::node::ToNode;
use crate::ast::tag::Partition;
use crate::result::runtime::ErrorKind::General;
use crate::result::Result;
use std::fmt;

#[derive(Debug, Clone)]
pub struct IfNode {
    pub condition: Box<Node>,
    pub true_branch: Box<Node>,
    pub false_branch: Box<Node>,
}

impl fmt::Display for IfNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} ? {:?} : {:?}",
            self.condition, self.true_branch, self.false_branch
        )
    }
}

impl ToNode for IfNode {
    fn make_node(tags: Vec<Node>) -> Result<Node> {
        let (_, condition, true_branch, false_branch, _) = tags.take_4().ok_or(General("Wow"))?;
        Ok(Node::If(IfNode {
            condition: Box::new(condition),
            true_branch: Box::new(true_branch),
            false_branch: Box::new(false_branch),
        }))
    }
}

impl IfNode {}
