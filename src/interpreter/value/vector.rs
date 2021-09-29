use crate::interpreter::Length;

use std::borrow::Borrow;
use std::cmp::max;
use std::fmt;
use std::fmt::Formatter;
use std::ops::Deref;
use std::sync::Arc;
use Node::*;
use View::*;

const INTERNAL_LINK_ERROR: &'static str = "Links should have at least one child.";
const LINK_INVARIANT: &'static str = "Expected an internal link";

impl<T> Length for Vector<T> {
    fn length(&self) -> usize {
        self.len()
    }
}

enum Operation<Ok, Value> {
    Success(Ok),
    WrongNodeOperation(Value),
    LeafFull(Value),
}

#[derive(Debug, Clone)]
enum Issue<T> {
    WrongNodeOperation,
    LeafFull(T),
}

type Result<T, K> = std::result::Result<T, Issue<K>>;

const BITS: usize = 5;
const SLOTS: usize = 1 << BITS;
const MASK: usize = SLOTS - 1;

#[derive(Debug, Clone)]
enum Node<T> {
    Leaf { values: Vec<T> },
    Internal { depth: usize, links: Vec<Link<T>> },
}

enum View<'a, T> {
    Values(&'a Vec<T>),
    Links(&'a Vec<Link<T>>),
}

enum Info {
    RightMostRoom,
    RoomInSelf,
    RoomInSubtree,
    Full,
}

impl<T> Node<T> {
    fn len(&self) -> usize {
        match self.view() {
            Values(vs) => vs.len(),
            Links(ls) => ls.len(),
        }
    }

    fn gen_leaf() -> Node<T> {
        Leaf { values: vec![] }
    }

    fn gen_internal(depth: usize) -> Node<T> {
        Internal {
            depth,
            links: vec![],
        }
    }

    fn decrease_depth(self) -> Node<T> {
        match self {
            Node::Leaf { values } => Leaf { values },
            Node::Internal { depth, links } => Internal {
                depth: max(depth - 1, 1),
                links,
            },
        }
    }

    fn info(&self) -> Info {
        match self.view() {
            Values(values) => {
                if values.len() < SLOTS {
                    Info::RightMostRoom
                } else {
                    Info::Full
                }
            }
            Links(links) => {
                let link = links.last().expect(INTERNAL_LINK_ERROR).as_ref();
                match link.info() {
                    Info::RightMostRoom => Info::RightMostRoom,
                    Info::RoomInSelf => Info::RoomInSubtree,
                    Info::RoomInSubtree => Info::RoomInSubtree,
                    Info::Full => {
                        if links.len() < SLOTS {
                            Info::RoomInSelf
                        } else {
                            Info::Full
                        }
                    }
                }
            }
        }
    }

    fn has_right_most_space(&self) -> bool {
        match self.view() {
            Values(values) => values.len() < SLOTS,
            Links(links) => links
                .last()
                .expect(INTERNAL_LINK_ERROR)
                .as_ref()
                .has_right_most_space(),
        }
    }

    fn is_full(&self) -> bool {
        match self.view() {
            Values(values) => values.len() >= SLOTS,
            Links(links) => {
                if links.len() < SLOTS {
                    false
                } else {
                    let last_link = self.last_link().unwrap();
                    last_link.is_full()
                }
            }
        }
    }

    fn links(&self) -> Option<&Vec<Link<T>>> {
        match self.view() {
            Links(links) => Some(links),
            _ => None,
        }
    }

    fn view(&self) -> View<T> {
        match self {
            Node::Leaf { values } => Values(values),
            Node::Internal { links, .. } => Links(links),
        }
    }

    fn last_link(&self) -> Option<&Link<T>> {
        self.links()?.last()
    }

    fn values(&self) -> Option<&Vec<T>> {
        match self {
            Node::Leaf { values } => Some(values),
            Node::Internal { .. } => None,
        }
    }

    fn depth(&self) -> usize {
        match self {
            Node::Leaf { .. } => 0,
            Node::Internal { depth, .. } => *depth,
        }
    }
    
    fn mask(&self, key: usize) -> usize {
        let offset = BITS * self.depth();
        let mask = MASK << offset;
        (key & mask) >> offset
    }

    fn get(&self, key: usize) -> Option<&T> {
        let index = self.mask(key);
        match self {
            Node::Leaf { values } => {
                let value = values.get(index)?;
                Some(value)
            }
            Node::Internal { links, .. } => {
                let link = links.get(index)?;
                link.get(key)
            }
        }
    }

    fn anchor(&self, value: T) -> Self
    where
        T: Clone,
    {
        match self {
            Node::Leaf { values } => {
                let mut values = values.clone();
                values.push(value);
                Leaf { values }
            }
            Node::Internal { depth, .. } => {
                let node = Arc::new(if *depth == 1 {
                    Node::gen_leaf().anchor(value)
                } else {
                    Node::gen_internal(depth - 1).anchor(value)
                });
                Internal {
                    depth: *depth,
                    links: vec![node],
                }
            }
        }
    }

    fn push(&self, value: T) -> Self
    where
        T: Clone,
    {
        match self.info() {
            Info::RightMostRoom => match self.view() {
                Values(vs) => {
                    let mut vs = vs.clone();
                    vs.push(value);
                    Leaf { values: vs }
                }
                Links(ls) => {
                    let last = ls.last().unwrap().as_ref();
                    let node = last.push(value);
                    let mut ls = ls.clone();
                    let last = ls.last_mut().unwrap();
                    *last = Arc::new(node);
                    Internal {
                        depth: self.depth(),
                        links: ls,
                    }
                }
            },
            Info::RoomInSelf => {
                let mut links = self.links().expect(LINK_INVARIANT).clone();
                if self.depth() == 1 {
                    links.push(Arc::new(Node::gen_leaf().anchor(value)))
                } else {
                    links.push(Arc::new(
                        Node::gen_internal(self.depth())
                            .decrease_depth()
                            .anchor(value),
                    ))
                }
                Internal {
                    depth: self.depth(),
                    links,
                }
            }
            Info::RoomInSubtree => {
                let mut links = self.links().expect(LINK_INVARIANT).clone();
                let new_link = links.pop().expect(INTERNAL_LINK_ERROR).push(value);
                links.push(Arc::new(new_link));
                Internal {
                    depth: self.depth(),
                    links,
                }
            }
            Info::Full => match self {
                Leaf { values: vs } => {
                    let links = vec![
                        Arc::new(Leaf { values: vs.clone() }),
                        Arc::new(Leaf {
                            values: vec![value],
                        }),
                    ];
                    Internal { depth: 1, links }
                }
                Internal { depth, links: ls } => {
                    let links = vec![
                        Arc::new(Internal {
                            depth: self.depth(),
                            links: ls.clone(),
                        }),
                        Arc::new(Node::gen_internal(*depth).anchor(value)),
                    ];
                    Internal {
                        depth: depth + 1,
                        links,
                    }
                }
            },
        }
    }
}

// impl<T> Drop for Node<T> {
//     fn drop(&mut self) {
//         println!("dropping") ;
//     }
// }

impl<T: fmt::Display> fmt::Display for Vector<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "( ")?;
        for i in 0..self.length {
            write!(f, "{} ", self.get(i).unwrap())?;
        }
        write!(f, ")")?;
        Ok(())
    }
}

impl<T: fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Leaf { values } => {
                let end = values.len();
                write!(f, "(")?;
                for (i, value) in values.iter().enumerate() {
                    write!(f, "{}", value)?;
                    if i + 1 != end {
                        write!(f, ",")?;
                    }
                }
                write!(f, ")")?;
            }
            Internal { depth: _, links } => {
                let end = links.len();
                write!(f, "(")?;
                for (i, link) in links.iter().enumerate() {
                    write!(f, "{}", link)?;
                    if i + 1 != end {
                        write!(f, " ")?;
                    }
                }
                write!(f, ")")?;
            }
        }
        Ok(())
    }
}

type Link<T> = Arc<Node<T>>;

#[derive(Debug, Clone)]
pub struct Vector<T> {
    length: usize,
    root: Node<T>,
}

impl<T> Vector<T> {
    pub fn len(&self) -> usize {
        self.length
    }

    pub fn new() -> Vector<T> {
        let primary = Leaf { values: vec![] };
        Vector {
            length: 0,
            root: primary,
        }
    }
    pub fn push(&self, value: T) -> Vector<T>
    where
        T: Clone,
    {
        Vector {
            length: self.length + 1,
            root: self.root.push(value),
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.root.get(index)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn update_leaf() -> Result<(), i32> {
        let mut leaf1 = Leaf { values: vec![] };
        let n: i32 = 100;
        for value in 0..n {
            leaf1 = leaf1.push(value);
        }
        for value in 0..n {
            assert_eq!(Some(&value), leaf1.get(value as usize))
        }
        Ok(())
    }

    #[test]
    fn vector() {
        let mut v: Vector<i32> = Vector::new();
        let n: i32 = 100000;
        for value in 0..n {
            v = v.push(value);
        }
        println!("{}", v.root);
        for value in 0..n {
            assert_eq!(Some(&value), v.get(value as usize))
        }
    }
}
