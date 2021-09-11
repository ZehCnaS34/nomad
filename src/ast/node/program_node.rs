use crate::ast::node::atom_node::AtomNode;
use crate::ast::parser::Tag;
use crate::ast::CHILD_LIMIT;
use crate::copy;
use crate::interpreter::{Execute, Interpreter};

#[derive(Debug, Copy, Clone)]
pub struct ProgramNode {
    expressions: [Tag; CHILD_LIMIT.program],
}

impl ProgramNode {
    pub fn from(tags: &[Tag]) -> ProgramNode {
        ProgramNode {
            expressions: copy! { tags, 0, CHILD_LIMIT.program },
        }
    }
}

impl Execute for ProgramNode {
    fn execute(&self, interpreter: &Interpreter, own_tag: Tag) -> AtomNode {
        for tag in Tag::tags(&self.expressions) {
            interpreter.interpret_tag(tag);
        }
        AtomNode::Nil
    }
}
