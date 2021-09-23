use crate::ast::node::Node;
use crate::ast::node::ToNode;
use crate::ast::tag::Partition;
use crate::ast::Tag;
use crate::interpreter::Interpreter;
use crate::result::parser::ErrorKind::General;
use crate::result::ParseResult;
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

impl ToNode for IfNode {
    fn make_node(tags: Vec<Tag>) -> ParseResult<Node> {
        let (_, condition, true_branch, false_branch, _) = tags.take_4().ok_or(General("Wow"))?;
        Ok(Node::If(IfNode {
            condition,
            true_branch,
            false_branch,
        }))
    }
}

impl IfNode {}
