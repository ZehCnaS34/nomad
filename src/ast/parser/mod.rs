use super::context::Context;
use super::node::atom_node;
use super::node::atom_node::{AtomNode, ParseError};
use super::node::def_node;
use super::node::def_node::DefinitionNode;
use super::node::function_node::FunctionCallNode;
use super::node::while_node::WhileNode;
use super::node::Node;
use super::scanner::token::{Kind, Token};
use crate::ast::context::FirstNodeAction;
use crate::ast::context::FirstNodeAction::IfExpression;
use crate::ast::node::atom_node::Symbol;
use crate::ast::node::if_node::IfNode;
use crate::list::List;
use std::cell::Cell;
use std::str::FromStr;

#[derive(Debug)]
pub enum Error {
    UnexpectedEof,
    ExpectedClosingParen,
    CouldNotParseAtom,
    IfMissingCondition,
    IfMissingTrueBranch,
    InvalidDefForm,
}

impl From<def_node::Error> for Error {
    fn from(def_node_error: def_node::Error) -> Error {
        Error::InvalidDefForm
    }
}

impl From<ParseError> for Error {
    fn from(parse_error: ParseError) -> Error {
        Error::CouldNotParseAtom
    }
}

pub type Result<T> = std::result::Result<T, Error>;

struct Parser {
    context: Context,
    position: Cell<usize>,
    tokens: Vec<Token>,
}

impl Parser {
    fn eof(&self) -> bool {
        let position = self.position.get();
        position >= self.tokens.len() || self.tokens[position].kind == Kind::Eof
    }

    fn next(&self) {
        self.position.set(self.position.get() + 1);
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position.get())
    }

    fn take(&self) -> Result<&Token> {
        let position = self.position.get();
        let result = self.tokens.get(position).ok_or(Error::UnexpectedEof)?;
        self.position.set(position + 1);
        Ok(result)
    }

    fn nested(&self) -> Result<Node> {
        let mut nodes = vec![];
        loop {
            let token = self.peek().ok_or(Error::UnexpectedEof)?;
            if token.kind == Kind::RightParen || token.kind == Kind::Eof {
                self.take();
                break;
            }
            nodes.push(self.expression()?);
        }
        match nodes.len() {
            0 => {
                todo!()
            }
            n => {
                // NOTE: Can panic...
                let mut nodes = nodes.into_iter();
                let function = nodes.next().unwrap();
                match self.context.match_form(&function) {
                    FirstNodeAction::Definition => {
                        Ok(Node::Definition(DefinitionNode::from_into_iter(nodes)?))
                    }
                    FirstNodeAction::IfExpression => {
                        let condition = nodes.next().ok_or(Error::IfMissingCondition)?;
                        let true_branch = nodes.next().ok_or(Error::IfMissingTrueBranch)?;
                        let result = Ok(Node::If(IfNode {
                            condition: Box::new(condition),
                            true_branch: Box::new(true_branch),
                            false_branch: Box::new(
                                nodes.next().unwrap_or(Node::Atom(AtomNode::Nil)),
                            ),
                        }));
                        println!("result {:#?}", result);
                        result
                    }
                    FirstNodeAction::FunctionCall => {
                        let arguments: Vec<_> = nodes.collect();
                        Ok(Node::FunctionCall(FunctionCallNode {
                            function: Box::new(function),
                            arguments,
                        }))
                    }
                    FirstNodeAction::WhileExpression => {
                        let condition = nodes.next().ok_or(Error::IfMissingCondition)?;
                        let body: Vec<_> = nodes.collect();
                        Ok(Node::While(WhileNode {
                            condition: Box::new(condition),
                            body,
                        }))
                    }
                    action => panic!("unsupported first node action {:?}", action),
                }
            }
        }
    }

    fn expression(&self) -> Result<Node> {
        let token = self.take()?;
        match token.kind {
            Kind::Symbol | Kind::String | Kind::Number => {
                let atom = AtomNode::from_str(&token.lexeme[..])?;
                Ok(Node::Atom(atom))
            }
            Kind::LeftParen => self.nested(),
            kind => {
                todo!("{:?}", kind);
            }
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Node>> {
    // this breaks.
    let def_form: Symbol = "def".into();
    let if_form: Symbol = "if".into();
    let fn_form: Symbol = "fn".into();
    let while_form: Symbol = "while".into();
    let mut nodes = vec![];
    let mut parser = Parser {
        context: Context::new(),
        position: Cell::new(0),
        tokens: tokens
            .into_iter()
            .filter(|token| token.kind != Kind::Comment)
            .collect(),
    };

    parser
        .context
        .set_form(def_form, FirstNodeAction::Definition);
    parser
        .context
        .set_form(if_form, FirstNodeAction::IfExpression);
    parser
        .context
        .set_form(fn_form, FirstNodeAction::FunctionDefinition);
    parser
        .context
        .set_form(while_form, FirstNodeAction::WhileExpression);

    while let Some(token) = parser.peek() {
        match token.kind {
            Kind::Comment => {
                parser.take();
            }
            Kind::Eof => {
                break;
            }
            _kind => {
                nodes.push(parser.expression()?);
            }
        }
    }

    Ok(nodes)
}
