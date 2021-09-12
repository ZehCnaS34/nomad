use std::collections::HashMap;
use std::rc::Rc;

pub trait Named {
    fn name(&self) -> String;
}

pub struct Graph<'a, T: Named> {
    nodes: HashMap<String, Node<'a, T>>,
}

struct Node<'a, T> {
    value: T,
    links: Vec<Rc<&'a Node<'a, T>>>,
}

impl<'a, T> Node<'a, T> {
    pub fn new(value: T) -> Node<'a, T> {
        Node {
            value,
            links: vec![],
        }
    }
}

impl<'a, T: Named> Graph<'a, T> {
    pub fn new() -> Graph<'a, T> {
        Graph {
            nodes: HashMap::new(),
        }
    }

    pub fn insert(&mut self, value: T) {
        self.nodes.insert(value.name(), Node::new(value));
    }

    pub fn link(&mut self, (from, to): (T, T)) -> Result<(), &'static str> {
        if self.nodes.contains_key(&from.name()) && self.nodes.contains_key(&to.name()) {
        } else {
            Err("Nodes not found in graph");
        }
    }
}
