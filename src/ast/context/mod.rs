use super::node;
use super::node::atom_node;
use super::node::atom_node::Symbol;
use atom_node::AtomNode;
use node::Node;
use std::cell::Cell;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub enum FirstNodeAction {
    FunctionCall,
    FunctionDefinition,
    Definition,
    ReverseIndex,
    Index,
    IfExpression,
    WhileExpression,
}

pub struct Context {
    symbol_actions: HashMap<Symbol, FirstNodeAction>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            symbol_actions: HashMap::new(),
        }
    }

    pub fn set_form(&mut self, symbol: Symbol, action: FirstNodeAction) {
        self.symbol_actions.insert(symbol, action);
    }

    pub fn match_form(&self, node: &node::Node) -> FirstNodeAction {
        match node {
            Node::Atom(atom) => match atom {
                AtomNode::Symbol(symbol) => self
                    .symbol_actions
                    .get(symbol)
                    .map(|action| action.clone())
                    .unwrap_or(FirstNodeAction::FunctionCall),
                AtomNode::Integer(_) => FirstNodeAction::ReverseIndex,
                AtomNode::String(_) => FirstNodeAction::Index,
                kind => {
                    panic!("First slot not supported");
                }
            },
            _ => FirstNodeAction::FunctionCall,
        }
    }
}
