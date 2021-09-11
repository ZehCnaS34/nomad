use std::cell::Cell;
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;
use std::sync::Mutex;

use crate::result::parser::ErrorKind as Error;
use crate::result::ParseResult as Result;

use super::node::{
    AtomNode, AtomParseError, DefinitionNode, DoNode, FunctionCallNode, FunctionNode, IfNode,
    LoopNode, Node, ProgramNode, RecurNode, VectorNode, WhileNode,
};
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

impl From<AtomParseError> for Error {
    fn from(parse_error: AtomParseError) -> Error {
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
            Node::Atom(..) => Tag::Atom(value),
            Node::Definition(..) => Tag::Definition(value),
            Node::Do(..) => Tag::Do(value),
            Node::FunctionCall(..) => Tag::Call(value),
            Node::Function(..) => Tag::Function(value),
            Node::If(..) => Tag::If(value),
            Node::Program(..) => Tag::Program(value),
            Node::Vector(..) => Tag::Vector(value),
            Node::While(..) => Tag::While(value),
            Node::Loop(..) => Tag::Loop(value),
            Node::Recur(..) => Tag::Recur(value),
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
                    "fn" if !symbol.is_qualified() => Form::Fn,
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
            Form::Fn => self.submit(Node::Function(FunctionNode::from_tags(&tags[1..]))),
            Form::Loop => self.submit(Node::Loop(LoopNode::from_tags(&tags[1..]))),
            Form::Recur => self.submit(Node::Recur(RecurNode::from_tags(&tags[1..]))),
            form => panic!("form {:?} not yet implemented", form),
        })
    }

    fn vector(&self) -> Result<Tag> {
        let mut tags = self.take_until(Kind::RightBracket)?;
        Ok(self.submit(Node::Vector(VectorNode::from_tags(&tags[..]))))
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
