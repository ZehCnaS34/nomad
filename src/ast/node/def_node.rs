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
    fn execute(&self, interpreter: &Interpreter, own_tag: Tag) {
        let ident = interpreter.intern_tag(self.ident);
        ident.take_symbol().map(|symbol| {
            if !symbol.is_qualified() {
                let value = interpreter.intern_tag(self.value);
                let value = interpreter.resolve(value);
                println!("setting symbol {:?} to {:?}", symbol, value);
                interpreter.define(symbol, value);
            }
        });
    }
}
