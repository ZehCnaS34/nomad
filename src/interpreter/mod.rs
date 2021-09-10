use super::ast::parser::AST;
use crate::ast::node::atom_node::{AtomNode, Symbol, Var};
use crate::ast::node::Node;
use crate::parser::Tag;
use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use std::sync::Mutex;

pub(crate) trait Introspection {
    fn is_truthy(&self) -> bool;
}

pub trait Execute {
    fn execute<'a>(&'a self, interpreter: &'a Interpreter, own_tag: Tag);
}

impl Introspection for AtomNode {
    fn is_truthy(&self) -> bool {
        match self {
            AtomNode::Boolean(b) => *b,
            _ => false,
        }
    }
}

pub trait Operation {
    type Val;
    type Err;
    fn add(&self, rhs: &Self) -> Result<Self::Val, Self::Err>;
    fn sub(&self, rhs: &Self) -> Result<Self::Val, Self::Err>;
    fn mul(&self, rhs: &Self) -> Result<Self::Val, Self::Err>;
    fn div(&self, rhs: &Self) -> Result<Self::Val, Self::Err>;
    fn eq(&self, rhs: &Self) -> Result<Self::Val, Self::Err>;
    fn lt(&self, rhs: &Self) -> Result<Self::Val, Self::Err>;
    fn gt(&self, rhs: &Self) -> Result<Self::Val, Self::Err>;
}

#[derive(Debug)]
pub struct Interpreter {
    ast: AST,
    values: Mutex<HashMap<Symbol, AtomNode>>,
    tag_data: Mutex<HashMap<Tag, AtomNode>>,
}

impl Interpreter {
    fn new(ast: AST) -> Interpreter {
        let mut queue = VecDeque::new();
        queue.push_back(ast.root.unwrap());
        Interpreter {
            ast,
            tag_data: Mutex::new(HashMap::new()),
            values: Mutex::new(HashMap::new()),
        }
    }

    pub fn add(&self, left: Tag, right: Tag) -> AtomNode {
        println!("left {:?} right {:?}", left, right);
        AtomNode::Boolean(false)
    }

    pub fn gt(&self, left: Tag, right: Tag) -> bool {
        println!("left {:?} right {:?}", left, right);
        true
    }

    pub fn lt(&self, left: Tag, right: Tag) -> bool {
        println!("left {:?} right {:?}", left, right);
        true
    }

    // fn resolve(&self, symbol: &Symbol) -> Option<&AtomNode> {
    //     let values = self.values.lock().unwrap();
    //     values.get(symbol).map(|value| {
    //         value
    //     })
    // }

    #[inline]
    pub(crate) fn put(&self, symbol: Symbol, atom: AtomNode) {
        let mut values = self.values.lock().unwrap();
        values.insert(symbol, atom);
    }

    pub fn is_tag_true(&self, tag: Tag) -> bool {
        true
    }

    #[inline]
    pub(crate) fn interpret_tag(&self, tag: Tag) {
        println!("tag {:?}", tag);
        let node = self.ast.get(&tag).unwrap();
        node.execute(&self, tag);
    }

    pub fn set_tag_data(&self, tag: Tag, data: AtomNode) {
        let mut tag_repo = self.tag_data.lock().unwrap();
        tag_repo.insert(tag, data);
    }

    pub fn resolve(&self, atom: AtomNode) -> AtomNode {
        match atom {
            AtomNode::Symbol(symbol) => {
                let mut values = self.values.lock().expect("Failed to lock values");
                values.get(&symbol).expect("Symbol does not exist.").clone()
            }
            atom => atom,
        }
    }

    pub fn define(&self, symbol: Symbol, value: AtomNode) {
        let mut values = self.values.lock().expect("Failed to lock values");
        values.insert(symbol, value);
    }

    pub fn intern_tag(&self, tag: Tag) -> AtomNode {
        self.interpret_tag(tag);
        let data = self.get_tag_data(tag);
        println!("{:?} {:?}", tag, data);
        data
    }

    pub fn get_tag_data(&self, tag: Tag) -> AtomNode {
        let mut tag_repo = self.tag_data.lock().unwrap();
        tag_repo.get(&tag).unwrap().clone()
    }

    fn run(&self) {
        if let Some(tag) = self.ast.root {
            println!("result = {:?}", self.interpret_tag(tag));
        }
        let tag_data = self.tag_data.lock().unwrap();
        let values = self.values.lock().unwrap();
        println!("{:#?}\n{:#?}", tag_data, values);
    }
}

pub fn interpret(ast: AST) {
    let env = Interpreter::new(ast);
    env.run();
}
