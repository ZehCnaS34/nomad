use std::cell::Cell;
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;
use std::sync::Mutex;

use crate::result::parser::ErrorKind as Error;
use crate::result::ParseResult as Result;

use super::node as n;
use super::scanner::token::{Kind, Token};
use super::CHILD_LIMIT;
use super::{Id, Tag};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Form {
    Call,
    Index,
    Macro,
    Special,
    While,
    If,
    Do,
    Def,
    Fn,
    Loop,
    Recur,
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

pub struct Parser {
    ast: Mutex<AST>,
    position: Cell<usize>,
    tokens: Vec<Token>,
}

impl Parser {
    #[inline]
    fn eof(&self) -> bool {
        let position = self.position.get();
        position >= self.tokens.len() || self.tokens[position].kind == Kind::Eof
    }

    #[inline]
    fn submit(&self, node: n::Node) -> Tag {
        let mut ast = self.ast.lock().unwrap();
        let value = ast.len();
        let tag = match &node {
            n::Node::Nil => Tag::Nil(value),
            n::Node::Boolean(..) => Tag::Boolean(value),
            n::Node::Number(..) => Tag::Number(value),
            n::Node::String(..) => Tag::String(value),
            n::Node::Symbol(..) => Tag::Symbol(value),
            n::Node::Definition(..) => Tag::Definition(value),
            n::Node::Do(..) => Tag::Do(value),
            n::Node::FunctionCall(..) => Tag::Call(value),
            n::Node::Function(..) => Tag::Function(value),
            n::Node::If(..) => Tag::If(value),
            n::Node::Program(..) => Tag::Program(value),
            n::Node::Vector(..) => Tag::Vector(value),
            n::Node::While(..) => Tag::While(value),
            n::Node::Loop(..) => Tag::Loop(value),
            n::Node::Recur(..) => Tag::Recur(value),
            node => panic!("node not yet implemented {:?}", node),
        };
        ast.insert(tag, node);
        return tag;
    }

    #[inline]
    fn next(&self) {
        self.position.set(self.position.get() + 1);
    }

    #[inline]
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position.get())
    }

    fn take(&self) -> Result<&Token> {
        let position = self.position.get();
        let result = self.tokens.get(position).ok_or(Error::UnexpectedEof)?;
        self.position.set(position + 1);
        Ok(result)
    }

    fn special_form(&self, tag: Tag) -> Form {
        tag.on_symbol()
            .and_then(|tag| {
                let ast = self.ast.lock().expect("Failed to lock ast mutex");
                ast.nodes
                    .get(tag)
                    .and_then(|node| node.as_symbol())
                    .map(|symbol| match symbol.name() {
                        "def" if !symbol.is_qualified() => Form::Def,
                        "loop" if !symbol.is_qualified() => Form::Loop,
                        "recur" if !symbol.is_qualified() => Form::Recur,
                        "while" if !symbol.is_qualified() => Form::While,
                        "if" if !symbol.is_qualified() => Form::If,
                        "do" if !symbol.is_qualified() => Form::Do,
                        "fn" if !symbol.is_qualified() => Form::Fn,
                        _ => Form::Call,
                    })
            })
            .unwrap_or(Form::Call)
    }

    fn take_until(&self, kind: Kind) -> Result<[Tag; 50]> {
        let mut nodes = [Tag::Noop; 50];
        let mut i = 0;
        loop {
            let token = self.peek().ok_or(Error::UnexpectedEof)?;
            if token.kind == kind || token.kind == Kind::Eof {
                self.take();
                break;
            }
            nodes[i] = self.expression()?;
            i += 1;
        }
        Ok(nodes)
    }

    fn nested(&self) -> Result<Tag> {
        let mut tags = self.take_until(Kind::RightParen)?;
        let first = tags[0];
        // println!("tags {:?}", tags);
        Ok(match self.special_form(first) {
            Form::Call => self.submit(n::Node::FunctionCall(n::FunctionCallNode::from_tags(
                &tags[..],
            ))),
            Form::While => self.submit(n::Node::While(n::WhileNode::from_tags(&tags[1..]))),
            Form::If => self.submit(n::Node::If(n::IfNode::from_tags(&tags[1..]))),
            Form::Def => self.submit(n::Node::Definition(n::DefinitionNode::from_tags(
                &tags[1..],
            ))),
            Form::Do => self.submit(n::Node::Do(n::DoNode::from_tags(&tags[1..]))),
            Form::Fn => self.submit(n::Node::Function(n::FunctionNode::from_tags(&tags[1..]))),
            Form::Loop => self.submit(n::Node::Loop(n::LoopNode::from_tags(&tags[1..]))),
            Form::Recur => self.submit(n::Node::Recur(n::RecurNode::from_tags(&tags[1..]))),
            form => panic!("form {:?} not yet implemented", form),
        })
    }

    fn vector(&self) -> Result<Tag> {
        let mut tags = self.take_until(Kind::RightBracket)?;
        Ok(self.submit(n::Node::Vector(n::VectorNode::from_tags(&tags[..]))))
    }

    fn expression(&self) -> Result<Tag> {
        let token = self.take()?;
        match token.kind {
            Kind::Symbol => match &token.lexeme[..] {
                "nil" => Ok(self.submit(n::Node::Nil)),
                "true" => Ok(self.submit(n::Node::Boolean(n::BooleanNode(true)))),
                "false" => Ok(self.submit(n::Node::Boolean(n::BooleanNode(false)))),
                lexeme => Ok(self.submit(n::Node::Symbol(n::SymbolNode::from(lexeme)))),
            },
            Kind::Number => {
                let number: f64 = (&token.lexeme[..]).parse().expect("Failed to parse number");
                Ok(self.submit(n::Node::Number(n::NumberNode(number))))
            }
            Kind::String => {
                let lexeme = &token.lexeme[..];
                Ok(self.submit(n::Node::String(n::StringNode::from(lexeme))))
            }
            Kind::LeftParen => self.nested(),
            Kind::LeftBracket => self.vector(),
            kind => {
                todo!("{:?}", kind);
            }
        }
    }

    fn program(&self) -> Result<Tag> {
        let mut expressions = [Tag::Noop; CHILD_LIMIT.program];
        let mut i = 0;
        while let Some(token) = self.peek() {
            match token.kind {
                Kind::Comment => {
                    self.take();
                }
                Kind::Eof => {
                    break;
                }
                _kind => {
                    expressions[i] = self.expression()?;
                    i += 1;
                }
            }
        }
        Ok(self.submit(n::Node::Program(n::ProgramNode::from(&expressions[..]))))
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<AST> {
    let mut parser = Parser {
        ast: Mutex::new(AST::new()),
        position: Cell::new(0),
        tokens: tokens
            .into_iter()
            .filter(|token| token.kind != Kind::Comment)
            .collect(),
    };

    parser.program();

    Ok(parser.ast.into_inner().unwrap())
}
