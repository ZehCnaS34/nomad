use crate::ast::node::Node;
use crate::ast::Tag;
use crate::ast::CHILD_LIMIT;
use crate::interpreter::Interpreter;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct IfNode {
    pub condition: Tag,
    pub true_branch: Tag,
    pub false_branch: Tag,
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

impl IfNode {
    pub fn from_tags(tags: &[Tag]) -> Self {
        IfNode {
            condition: tags[0],
            true_branch: tags[1],
            false_branch: tags[2],
        }
    }
}
