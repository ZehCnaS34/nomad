use crate::ast::node::{AtomNode, Node, Symbol};
use crate::ast::Tag;
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
        let atom = interpreter
            .get_atom_node(self.ident)
            .expect("First slot of a definition node must be an atom");
        let symbol = atom.as_symbol().expect("Ident must be a symbol");
        if symbol.is_qualified() {
            panic!("Definition nodes must be an unqualified symbol");
        }
        interpreter.define(symbol.clone(), interpreter.interpret_tag(self.value))
    }
}
