use crate::ast::node::atom_node::{AtomNode, Symbol};
use crate::ast::node::Node;
use crate::ast::parser::Tag;
use crate::ast::CHILD_LIMIT;
use crate::interpreter::{Execute, Interpreter};
use std::fmt;
use std::vec::IntoIter;

#[derive(Debug, Copy, Clone)]
pub struct DefinitionNode {
    ident: Tag,
    value: Tag,
}

impl DefinitionNode {
    pub fn from_tags(tags: &[Tag]) -> DefinitionNode {
        let ident = tags[0];
        let value = tags[1];
        DefinitionNode { ident, value }
    }
}

impl Execute for DefinitionNode {
    fn execute(&self, interpreter: &Interpreter, own_tag: Tag) -> AtomNode {
        interpreter
            .get_atom_node(self.ident)
            .and_then(|atom| atom.as_symbol())
            .map(|symbol| {
                if symbol.is_qualified() {
                    panic!("ident cannot be qualifed")
                } else {
                    interpreter.define(symbol.clone(), interpreter.interpret_tag(self.value))
                }
            })
            .expect("Ident must be a symbol")
    }
}
