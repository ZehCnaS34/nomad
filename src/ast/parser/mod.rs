use std::cell::Cell;
use std::str::FromStr;

// use super::node::Node;
use super::node::atom_node;
use super::node::atom_node::{AtomNode, ParseError};
use super::node::def_node;
use super::node::def_node::DefinitionNode;
use super::node::function_node::FunctionCallNode;
use super::node::while_node::WhileNode;
use super::node::Node;
use super::scanner::token::{Kind, Token};
use std::fmt;

use crate::ast::node::atom_node::Symbol;
use crate::ast::node::if_node::IfNode;
use im::HashMap;
use std::collections::VecDeque;
use std::fmt::Formatter;
use std::sync::Mutex;

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

type Id = usize;
type Ids = Vec<Id>;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum NodeKind {
    Atom,
    Definition,
    While,
    If,
    Call,
    Program,
    Vector,
    Do,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct Tag {
    kind: NodeKind,
    value: usize,
}

impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.kind {
            NodeKind::Atom => write!(f, "[atom::{}]", self.value),
            NodeKind::While => write!(f, "[while::{}]", self.value),
            NodeKind::Definition => write!(f, "[def::{}]", self.value),
            NodeKind::Call => write!(f, "[call::{}]", self.value),
            NodeKind::Program => write!(f, "[program::{}]", self.value),
            NodeKind::Vector => write!(f, "[vector::{}]", self.value),
            NodeKind::Do => write!(f, "[do::{}]", self.value),
            NodeKind::If => write!(f, "[if::{}]", self.value),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    Atom(AtomNode),
    While {
        condition: Tag,
        body: Vec<Tag>,
    },
    Definition {
        ident: Tag,
        value: Tag,
    },
    Call {
        function: Tag,
        arguments: Vec<Tag>,
    },
    Program {
        expressions: Vec<Tag>,
    },
    Vector {
        expressions: Vec<Tag>,
    },
    Do {
        expressions: Vec<Tag>,
    },
    If {
        condition: Tag,
        then: Tag,
        otherwise: Tag,
    },
}

#[derive(Debug)]
pub struct AST {
    nodes: HashMap<Tag, Node>,
    pub root: Option<Tag>,
}

pub struct AstIter<'a> {
    ast: &'a AST,
    queue: VecDeque<Tag>,
}

impl<'a> Iterator for AstIter<'a> {
    type Item = (Tag, &'a AtomNode);

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front().and_then(|tag| {
            self.ast.get(&tag).and_then(|node| match &node {
                Node::Atom(atom) => Some((tag, atom)),
                Node::While { condition, body } => {
                    self.queue.push_back(*condition);
                    for e in body {
                        self.queue.push_back(*e);
                    }
                    self.next()
                }
                Node::Definition { ident, value } => {
                    self.queue.push_back(*ident);
                    self.queue.push_back(*value);
                    self.next()
                }
                Node::Call {
                    function,
                    arguments,
                } => {
                    self.queue.push_back(*function);
                    for e in arguments {
                        self.queue.push_back(*e);
                    }
                    self.next()
                }
                Node::Program { expressions } => {
                    for e in expressions {
                        self.queue.push_back(*e);
                    }
                    self.next()
                }
                Node::Vector { expressions } => {
                    for e in expressions {
                        self.queue.push_back(*e);
                    }
                    self.next()
                }
                Node::Do { expressions } => {
                    for e in expressions {
                        self.queue.push_back(*e);
                    }
                    self.next()
                }
                Node::If {
                    condition,
                    then,
                    otherwise,
                } => {
                    self.queue.push_back(*condition);
                    self.queue.push_back(*then);
                    self.queue.push_back(*otherwise);
                    self.next()
                }
            })
        })
    }
}

impl AST {
    fn new() -> AST {
        AST {
            nodes: HashMap::new(),
            root: None,
        }
    }

    pub fn iter(&self) -> AstIter {
        let mut queue = VecDeque::new();
        if let Some(tag) = self.root {
            queue.push_back(tag);
        }
        AstIter { ast: &self, queue }
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
        let id = match &node {
            Node::Do { .. } => Tag {
                kind: NodeKind::Do,
                value: ast.len(),
            },
            Node::Vector { .. } => Tag {
                kind: NodeKind::Vector,
                value: ast.len(),
            },
            Node::Program { .. } => Tag {
                kind: NodeKind::Program,
                value: ast.len(),
            },
            Node::Call { .. } => Tag {
                kind: NodeKind::Call,
                value: ast.len(),
            },
            Node::While { .. } => Tag {
                kind: NodeKind::While,
                value: ast.len(),
            },
            Node::If { .. } => Tag {
                kind: NodeKind::If,
                value: ast.len(),
            },
            Node::Atom { .. } => Tag {
                kind: NodeKind::Atom,
                value: ast.len(),
            },
            Node::Definition { .. } => Tag {
                kind: NodeKind::Definition,
                value: ast.len(),
            },
        };
        ast.insert(id, node);
        return id;
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

    fn special_form(&self, tag: Tag) -> NodeKind {
        let ast = self.ast.lock().unwrap();
        match ast.nodes.get(&tag) {
            None => NodeKind::Call,
            Some(node) => match node {
                Node::Atom(atom) => match atom {
                    AtomNode::Symbol(symbol) if !symbol.is_qualified() => match symbol.name() {
                        "def" => NodeKind::Definition,
                        "if" => NodeKind::If,
                        "while" => NodeKind::While,
                        "do" => NodeKind::Do,
                        _ => NodeKind::Call,
                    },
                    _ => NodeKind::Call,
                },
                _ => NodeKind::Call,
            },
        }
    }

    fn take_until(&self, kind: Kind) -> Result<Vec<Tag>> {
        let mut nodes = vec![];
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

    fn nested(&self) -> Result<Tag> {
        let mut nodes = self.take_until(Kind::RightParen)?.into_iter();
        let function = nodes.next().unwrap();
        match self.special_form(function) {
            NodeKind::Definition => {
                let ident = nodes.next().unwrap();
                let value = nodes.next().unwrap();
                assert_eq!(0, nodes.len());
                Ok(self.submit(Node::Definition { ident, value }))
            }
            NodeKind::While => Ok(self.submit(Node::While {
                condition: nodes.next().unwrap(),
                body: nodes.collect(),
            })),
            NodeKind::If => {
                let condition = nodes.next().unwrap();
                let then = nodes.next().unwrap();
                let otherwise = nodes.next().unwrap();
                Ok(self.submit(Node::If {
                    condition,
                    then,
                    otherwise,
                }))
            }
            NodeKind::Do => Ok(self.submit(Node::Do {
                expressions: nodes.collect(),
            })),
            NodeKind::Call => Ok(self.submit(Node::Call {
                function,
                arguments: nodes.collect(),
            })),
            kind => {
                panic!("What is this {:?}")
            }
        }
    }

    fn vector(&self) -> Result<Tag> {
        let mut expressions = self.take_until(Kind::RightBracket)?;
        Ok(self.submit(Node::Vector { expressions }))
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
        let mut expressions = vec![];
        while let Some(token) = self.peek() {
            match token.kind {
                Kind::Comment => {
                    self.take();
                }
                Kind::Eof => {
                    break;
                }
                _kind => {
                    expressions.push(self.expression()?);
                }
            }
        }
        Ok(self.submit(Node::Program { expressions }))
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
