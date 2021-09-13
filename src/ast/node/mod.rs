use crate::ast::Tag;
use crate::interpreter::Interpreter;

mod boolean_node;
mod def_node;
mod do_node;
mod function_node;
mod if_node;
mod keyword_node;
mod list_node;
mod loop_node;
mod number_node;
mod program_node;
mod recur_node;
mod string_node;
mod symbol_node;
mod vector_node;
mod while_node;

pub use boolean_node::BooleanNode;
pub use def_node::DefinitionNode;
pub use do_node::DoNode;
pub use function_node::FunctionCallNode;
pub use function_node::FunctionNode;
pub use if_node::IfNode;
pub use keyword_node::KeywordNode;
pub use list_node::ListNode;
pub use loop_node::LoopNode;
pub use number_node::NumberNode;
pub use program_node::ProgramNode;
pub use recur_node::RecurNode;
pub use string_node::StringNode;
pub use symbol_node::SymbolNode;
pub use vector_node::VectorNode;
pub use while_node::WhileNode;

#[derive(Debug, Clone)]
pub enum Node {
    Nil,
    Boolean(BooleanNode),
    Number(NumberNode),
    String(StringNode),
    Symbol(SymbolNode),
    Keyword(KeywordNode),
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
    pub fn as_symbol(&self) -> Option<&SymbolNode> {
        match self {
            Node::Symbol(symbol) => Some(symbol),
            _ => None,
        }
    }

    pub fn take_vector(self) -> Option<VectorNode> {
        match self {
            Node::Vector(node) => Some(node),
            _ => None,
        }
    }
}
