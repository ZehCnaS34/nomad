use crate::ast::node::Node;
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
            "{} ? {} : {}",
            self.condition, self.true_branch, self.false_branch
        )
    }
}
