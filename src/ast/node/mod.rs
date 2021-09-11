use crate::ast::Tag;
use crate::interpreter::{Execute, Interpreter};

mod atom_node;
mod def_node;
mod do_node;
mod function_node;
mod if_node;
mod list_node;
mod loop_node;
mod program_node;
mod recur_node;
mod vector_node;
mod while_node;

pub use atom_node::{AtomNode, AtomParseError, Symbol, ToRational, ToSymbol, Var};
pub use def_node::DefinitionNode;
pub use do_node::DoNode;
pub use function_node::{FunctionCallNode, FunctionNode};
pub use if_node::IfNode;
pub use list_node::ListNode;
pub use loop_node::LoopNode;
pub use program_node::ProgramNode;
pub use recur_node::RecurNode;
pub use vector_node::VectorNode;
pub use while_node::WhileNode;

#[derive(Debug, Clone)]
pub enum Node {
    /// Represents a constant value in nomad.
    /// things like number, string, symbol
    Atom(AtomNode),
    /// Represents an anonymous or named function
    Function(FunctionNode),
    FunctionCall(FunctionCallNode),
    Definition(DefinitionNode),
    If(IfNode),
    While(WhileNode),
    List(ListNode),
    Do(DoNode),
    Program(ProgramNode),
    Vector(VectorNode),
    Recur(RecurNode),
    Loop(LoopNode),
}

impl Node {
    pub fn as_atom(&self) -> Option<&atom_node::AtomNode> {
        match self {
            Node::Atom(node) => Some(node),
            _ => None,
        }
    }
}

impl Execute for Node {
    fn execute(&self, interpreter: &Interpreter, own_tag: Tag) -> AtomNode {
        match self {
            Node::Atom(node) => node.execute(interpreter, own_tag),
            Node::Function(node) => node.execute(interpreter, own_tag),
            Node::FunctionCall(node) => node.execute(interpreter, own_tag),
            Node::Definition(node) => node.execute(interpreter, own_tag),
            Node::If(node) => node.execute(interpreter, own_tag),
            Node::While(node) => node.execute(interpreter, own_tag),
            Node::List(node) => node.execute(interpreter, own_tag),
            Node::Do(node) => node.execute(interpreter, own_tag),
            Node::Program(node) => node.execute(interpreter, own_tag),
            Node::Vector(node) => node.execute(interpreter, own_tag),
            Node::Loop(node) => node.execute(interpreter, own_tag),
            Node::Recur(node) => node.execute(interpreter, own_tag),
        }
    }
}
