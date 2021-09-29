use crate::defnode;
use crate::prelude::*;

use crate::ast::node::{Node, SymbolNode, ToNode};
use crate::ast::tag::Partition;
use crate::ast::Tag;
use crate::interpreter::Interpreter;
use crate::result::runtime::ErrorKind;
use std::fmt;
use std::vec::IntoIter;

#[derive(Debug, Clone)]
pub struct DefinitionNode {
    ident: SymbolNode,
    value: Box<Node>,
}

impl DefinitionNode {
    pub fn ident(&self) -> &SymbolNode {
        // &self.ident
        &self.ident
    }

    pub fn value(&self) -> &Node {
        self.value.as_ref()
    }
}

defnode! {
    Node::Definition : DefinitionNode :: nodes => {
        let (_, ident, value, rest) = nodes.take_3().ok_or(CouldNotParseAtom)?;
        let ident = ident.take_symbol().ok_or(General("fuck"))?;
        Ok(DefinitionNode {
            ident,
            value: Box::new(value),
        })
    }
}
