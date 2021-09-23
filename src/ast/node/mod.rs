use crate::ast::Tag;
use crate::interpreter::Interpreter;

mod boolean_node;
mod decorator_node;
mod def_node;
mod do_node;
mod function_node;
mod if_node;
mod keyword_node;
mod let_node;
mod list_node;
mod loop_node;
mod macro_node;
mod meta_node;
mod number_node;
mod program_node;
mod quasi_quote_node;
mod quote_node;
mod recur_node;
mod string_node;
mod symbol_node;
mod vector_node;
mod while_node;

pub use boolean_node::BooleanNode;
pub use decorator_node::DecoratorNode;
pub use def_node::DefinitionNode;
pub use do_node::DoNode;
pub use function_node::FunctionCallNode;
pub use function_node::FunctionNode;
pub use if_node::IfNode;
pub use keyword_node::KeywordNode;
pub use let_node::LetNode;
pub use list_node::ListNode;
pub use loop_node::LoopNode;
pub use macro_node::MacroNode;
pub use meta_node::MetaNode;
pub use number_node::NumberNode;
pub use program_node::ProgramNode;
pub use quasi_quote_node::QuasiQuoteNode;
pub use quote_node::QuoteNode;
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
    Let(LetNode),
    List(ListNode),
    Do(DoNode),
    Program(ProgramNode),
    Vector(VectorNode),
    Recur(RecurNode),
    Loop(LoopNode),
    Quote(QuoteNode),
    QuasiQuote(QuasiQuoteNode),
    Meta(MetaNode),
    Macro(MacroNode),
    Decorator(DecoratorNode),
}

pub trait ToNode {
    fn make_node(tags: Vec<Tag>) -> Result<Node, crate::result::parser::ErrorKind>;
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
