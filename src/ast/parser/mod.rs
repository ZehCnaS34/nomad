use std::cell::Cell;
use std::collections::{HashMap, VecDeque};
use std::convert::TryFrom;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;
use std::sync::Mutex;

use crate::result::runtime::ErrorKind as Error;
use crate::result::RuntimeResult as Result;

use super::node as n;
use super::node::ToNode;
use super::scanner::token::{Kind, Token};
use super::{Id, Tag};
use crate::ast::node::QuoteNode;
use crate::ast::tag::TagKind;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Form {
    Call,
    Index,
    Macro,
    Special,
    While,
    If,
    Do,
    Let,
    Def,
    Fn,
    Loop,
    Recur,
    QuasiQuote,
}

#[derive(Debug)]
pub struct AST {
    nodes: HashMap<Tag, n::Node>,
    pub root: Option<Tag>,
}

impl AST {
    fn new() -> AST {
        AST {
            nodes: HashMap::new(),
            root: None,
        }
    }

    pub fn get(&self, tag: &Tag) -> Option<&n::Node> {
        self.nodes.get(tag)
    }

    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn insert(&mut self, id: Tag, node: n::Node) {
        self.root = Some(id);
        self.nodes.insert(id, node);
    }
}

#[derive(Debug)]
pub struct Parser {
    ast: Mutex<AST>,
    position: Cell<usize>,
    tokens: Vec<Token>,
}

impl Parser {
    fn eof(&self) -> bool {
        let position = self.position.get();
        position >= self.tokens.len() || self.tokens[position].kind == Kind::Eof
    }

    pub fn get_node(&self, tag: Tag) -> Option<n::Node> {
        let ast = self.ast.lock().ok()?;
        ast.get(&tag).cloned()
    }

    fn submit(&self, node: n::Node) -> Result<n::Node> {
        return Ok(node);
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

    fn special_form(&self, node: &n::Node) -> Form {
        if let Some(node) = node.as_symbol() {
            match node.name() {
                "def" if !node.is_qualified() => Form::Def,
                "loop" if !node.is_qualified() => Form::Loop,
                "recur" if !node.is_qualified() => Form::Recur,
                "while" if !node.is_qualified() => Form::While,
                "if" if !node.is_qualified() => Form::If,
                "do" if !node.is_qualified() => Form::Do,
                "fn" if !node.is_qualified() => Form::Fn,
                "let*" if !node.is_qualified() => Form::Let,
                "defmacro" if !node.is_qualified() => Form::Macro,
                _ => Form::Call,
            }
        } else {
            Form::Call
        }
    }

    fn take_until(&self, kind: Kind) -> Result<Vec<n::Node>> {
        let mut nodes = Vec::new();
        loop {
            let token = self.peek().ok_or(Error::UnexpectedEof)?;
            if token.kind == kind || token.kind == Kind::Eof {
                self.take();
                break;
            }
            nodes.push(self.expression()?);
        }
        Ok(nodes)
    }

    fn nested(&self) -> Result<n::Node> {
        let mut tags = self.take_until(Kind::RightParen)?;

        self.submit(match self.special_form(&tags[0]) {
            Form::Call => n::FunctionCallNode::make_node(tags),
            Form::While => n::WhileNode::make_node(tags),
            Form::If => n::IfNode::make_node(tags),
            Form::Def => n::DefinitionNode::make_node(tags),
            Form::Do => n::DoNode::make_node(tags),
            Form::Fn => n::FunctionNode::make_node(tags),
            Form::Loop => n::LoopNode::make_node(tags),
            Form::Recur => n::RecurNode::make_node(tags),
            Form::Let => n::LetNode::make_node(tags),
            Form::Macro => todo!("Macro node not implemented"),
            Form::Index => todo!("implement index"),
            Form::Special => todo!("special form"),
            Form::QuasiQuote => todo!("quasi quote"),
        }?)
    }

    fn vector(&self) -> Result<n::Node> {
        Ok(n::Node::Vector(n::VectorNode::try_from(
            self.take_until(Kind::RightBracket)?,
        )?))
    }

    // fn escape_list(&self) -> Result<Tag> {
    //     let item = self.expression()?;
    //     self.submit(n::Node::QuasiQuote(n::QuasiQuoteNode::from_tag(item)))
    // }

    // fn unquote(&self) -> Result<Tag> {
    //     let tag = self.expression()?;
    //     self.submit(n::Node::Quote(QuoteNode::from_tag(tag)))
    // }

    // fn quote(&self) -> Result<Tag> {
    //     let expression = self.expression()?;
    //     self.submit(n::Node::Quote(n::QuoteNode::from_tag(expression)))
    // }

    // fn carrot(&self) -> Result<Tag> {
    //     self.submit(n::MetaNode::make_node(vec![
    //         self.expression()?,
    //         self.expression()?,
    //     ])?)
    // }

    // fn decorator(&self) -> Result<Tag> {
    //     self.submit(n::DecoratorNode::make_node(vec![
    //         self.expression()?,
    //         self.expression()?,
    //     ])?)
    // }

    fn expression(&self) -> Result<n::Node> {
        let token = self.take()?;
        match token.kind {
            Kind::Symbol => match &token.lexeme[..] {
                "nil" => self.submit(n::Node::Nil),
                "true" => self.submit(n::Node::Boolean(n::BooleanNode(true))),
                "false" => self.submit(n::Node::Boolean(n::BooleanNode(false))),
                lexeme => self.submit(n::Node::Symbol(n::SymbolNode::from(lexeme))),
            },
            Kind::Number => {
                let number: f64 = (&token.lexeme[..]).parse().expect("Failed to parse number");
                self.submit(n::Node::Number(n::NumberNode(number)))
            }
            Kind::String => {
                let lexeme = &token.lexeme[..];
                self.submit(n::Node::String(n::StringNode::from(lexeme)))
            }
            // Kind::Carrot => self.carrot(),
            // Kind::Quote => self.quote(),
            // Kind::Hash => self.decorator(),
            Kind::LeftParen => self.nested(),
            Kind::LeftBracket => self.vector(),
            // Kind::BackTick => self.escape_list(),
            // Kind::Unquote => self.unquote(),
            kind => {
                todo!("{:?}", kind);
            }
        }
    }

    fn program(&self) -> Result<n::Node> {
        let mut expressions = Vec::new();
        while let Some(token) = self.peek() {
            match token.kind {
                Kind::Comment => {
                    self.take();
                }
                Kind::Eof => {
                    break;
                }
                _kind => {
                    let e = self.expression()?;
                    expressions.push(e);
                }
            }
        }
        self.submit(n::Node::Program(n::ProgramNode::new(expressions)))
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<n::Node> {
    let mut parser = Parser {
        ast: Mutex::new(AST::new()),
        position: Cell::new(0),
        tokens: tokens
            .into_iter()
            .filter(|token| token.kind != Kind::Comment)
            .collect(),
    };

    return parser.program();
}
