use super::Value;
use std::cell::Cell;
use std::cmp::max;
use std::fmt;
use std::fmt::Formatter;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

const SIZE: usize = 4;

lazy_static! {
    static ref COUNT: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
}

fn make_id() -> i32 {
    let mut count = COUNT.try_lock().unwrap();
    *count += 1;
    count.clone()
}

fn reset_id() {
    let mut count = COUNT.try_lock().unwrap();
    *count = 0;
}

#[derive(Debug, Clone)]
struct Vector {
    root: Link,
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}}}", self.root)
    }
}

#[derive(Debug, Clone)]
struct Node {
    id: i32,
    depth: usize,
    value: Option<i32>,
    links: Vec<Link>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(value) = self.value {
            write!(f, "({})", value)
        } else {
            write!(f, "[")?;
            let last = self.links.len() - 1;
            for (i, slot) in self.links.iter().enumerate() {
                write!(f, "{}", slot)?;
                if i != last {
                    write!(f, " ")?;
                }
            }
            write!(f, "]")
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Node {
            id: make_id(),
            depth: 0,
            value: None,
            links: Vec::with_capacity(SIZE),
        }
    }
}

impl Node {
    fn push_node(&mut self, node: Rc<Node>) {
        self.links.push(node);
    }

    fn is_leaf(&self) -> bool {
        self.value.is_some()
    }

    fn push(&mut self, value: i32) {
        self.links.push(Rc::new(Node {
            id: make_id(),
            depth: 0,
            value: Some(value),
            links: Vec::with_capacity(SIZE),
        }));
    }

    fn is_full(&self) -> bool {
        if self.value.is_some() {
            return true;
        } else if self.links.len() < SIZE {
            return false;
        } else {
            self.links[SIZE - 1].is_full()
        }
    }
}

type Link = Rc<Node>;

impl Vector {
    fn empty() -> Vector {
        Vector {
            root: Rc::new(Node::default()),
        }
    }

    fn conj(&self, value: i32) -> Vector {
        let node = self.root.clone();
        let result = if node.is_full() {
            println!("splitting {}", value);
            // use reference
            let mut new_root = Node::default();
            let max_depth = node.depth + 1;
            new_root.depth = max_depth;
            new_root.push_node(node);
            let mut new_leaf = Node::default();
            new_leaf.push(value);
            for depth in 1..max_depth {
                let mut inter = Node::default();
                inter.depth = depth;
                inter.push_node(Rc::new(new_leaf));
                new_leaf = inter;
            }
            new_root.push_node(Rc::new(new_leaf));
            Vector {
                root: Rc::new(new_root),
            }
        } else {
            println!("cloning {}", value);
            // use clone
            let mut new_root = node.as_ref().clone();
            new_root.id = make_id();
            new_root.push(value);
            Vector {
                root: Rc::new(new_root),
            }
        };
        println!("v {:#?}", result);
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn empty() {
        reset_id();
        let node = Node::default();
        assert_eq!(node.is_full(), false);
        assert_eq!(node.depth, 0);
    }
    #[test]
    fn conj() {
        reset_id();
        let mut node = Node::default();
        node.push(23);
        println!("node {:?}", node);
        assert_eq!(node.is_full(), false);
        assert_eq!(node.depth, 0);
    }

    #[test]
    fn vec() {
        reset_id();
        let list = Vector::empty()
            .conj(1)
            .conj(2)
            .conj(3)
            .conj(1)
            .conj(2)
            .conj(3);
        println!("{}", list);
    }
}
