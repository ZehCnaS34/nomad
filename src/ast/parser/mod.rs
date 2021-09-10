use crate::ast::node::do_node::DoNode;
use crate::ast::node::program_node::ProgramNode;
use std::cell::Cell;
use std::str::FromStr;

// use super::node::Node;
use super::node::atom_node;
use super::node::atom_node::{AtomNode, ParseError};
use super::node::def_node;
use super::node::def_node::DefinitionNode;
use super::node::function_node::FunctionCallNode;
use super::node::while_node::WhileNode;
use super::scanner::token::{Kind, Token};
use crate::result::parser::ErrorKind as Error;
use crate::result::ParseResult as Result;
use std::fmt;

use crate::ast::node::atom_node::Symbol;
use crate::ast::node::if_node::IfNode;
use crate::ast::node::Node;
use crate::ast::CHILD_LIMIT;
use im::HashMap;
use std::collections::VecDeque;
use std::fmt::Formatter;
use std::sync::Mutex;

type Id = usize;
type Ids = Vec<Id>;

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
    Loop,
    Recur,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Tag {
    Noop,
    Atom(Id),
    Definition(Id),
    While(Id),
    If(Id),
    Call(Id),
    Program(Id),
    Vector(Id),
    Let(Id),
    Do(Id),
}

pub struct TagIter<'a> {
    current: usize,
    tags: &'a [Tag],
}

impl Tag {
    pub fn tags(tags: &[Tag]) -> TagIter {
        TagIter { tags, current: 0 }
    }
}

impl<'a> Iterator for TagIter<'a> {
    type Item = Tag;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.tags.len() {
            return None;
        }
        let tag = self.tags[self.current];
        if tag == Tag::Noop {
            return None;
        }
        self.current += 1;
        Some(tag)
    }
}

impl From<atom_node::ParseError> for Error {
    fn from(parse_error: atom_node::ParseError) -> Error {
        Error::CouldNotParseAtom
    }
}

#[derive(Debug)]
pub struct AST {
    nodes: HashMap<Tag, Node>,
    pub root: Option<Tag>,
}

impl AST {
    fn new() -> AST {
        AST {
            nodes: HashMap::new(),
            root: None,
        }
    }

    pub fn get(&self, tag: &Tag) -> Option<&Node> {
        self.nodes.get(tag)
    }

    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn insert(&mut self, id: Tag, node: Node) {
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
    fn submit(&self, node: Node) -> Tag {
        let mut ast = self.ast.lock().unwrap();
        let value = ast.len();
        let tag = match &node {
            Node::Do(..) => Tag::Do(value),
            Node::Program(..) => Tag::Program(value),
            Node::FunctionCall(..) => Tag::Call(value),
            Node::While(..) => Tag::While(value),
            Node::If(..) => Tag::If(value),
            Node::Atom(..) => Tag::Atom(value),
            Node::Definition(..) => Tag::Definition(value),
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
        let ast = self.ast.lock().unwrap();
        match ast.nodes.get(&tag).unwrap() {
            Node::Atom(atom) => match atom {
                AtomNode::Symbol(symbol) => match symbol.name() {
                    "def" if !symbol.is_qualified() => Form::Def,
                    "loop" if !symbol.is_qualified() => Form::Loop,
                    "recur" if !symbol.is_qualified() => Form::Recur,
                    "while" if !symbol.is_qualified() => Form::While,
                    "if" if !symbol.is_qualified() => Form::If,
                    "do" if !symbol.is_qualified() => Form::Do,
                    _ => Form::Call,
                },
                _ => Form::Call,
            },
            _ => Form::Call,
        }
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
            Form::Call => self.submit(Node::FunctionCall(FunctionCallNode::from_tags(&tags[..]))),
            Form::While => self.submit(Node::While(WhileNode::from_tags(&tags[1..]))),
            Form::If => self.submit(Node::If(IfNode::from_tags(&tags[1..]))),
            Form::Def => self.submit(Node::Definition(DefinitionNode::from_tags(&tags[1..]))),
            Form::Do => self.submit(Node::Do(DoNode::from_tags(&tags[1..]))),
            form => panic!("form {:?} not yet implemented", form),
        })
    }

    fn expression(&self) -> Result<Tag> {
        let token = self.take()?;
        match token.kind {
            Kind::Symbol | Kind::String | Kind::Number => {
                let atom = AtomNode::from_str(&token.lexeme[..])?;
                let id = self.submit(Node::Atom(atom));
                Ok(id)
            }
            Kind::LeftParen => self.nested(),
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
        Ok(self.submit(Node::Program(ProgramNode::from(&expressions[..]))))
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
