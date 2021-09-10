use crate::ast::node::atom_node::AtomNode;
use crate::ast::node::Node;
use crate::ast::parser::Tag;
use crate::ast::CHILD_LIMIT;
use crate::interpreter::{Execute, Interpreter};
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

impl Execute for IfNode {
    fn execute(&self, interpreter: &Interpreter, own_tag: Tag) {
        interpreter.interpret_tag(self.condition);
        if interpreter.is_tag_true(self.condition) {
            interpreter.interpret_tag(self.true_branch);
        } else {
            interpreter.interpret_tag(self.false_branch);
        }
    }
}
