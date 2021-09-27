use std::borrow::Borrow;
use std::fmt;
use std::fmt::Formatter;
use std::ops::Deref;
use std::sync::Arc;
use Node::*;
use View::*;

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

const BITS: usize = 2;
const SLOTS: usize = BITS.pow(2);
const MASK: usize = (1 << BITS) - 1;

#[derive(Debug, Clone)]
enum Node<T> {
    Leaf { values: Vec<Arc<T>> },
    Internal { depth: usize, links: Vec<Link<T>> },
}

enum View<'a, T> {
    Values(&'a Vec<Arc<T>>),
    Links(&'a Vec<Link<T>>),
}

enum Info {
    RightMostRoom,
    RoomInSelf,
    Full,
}

impl<T> Node<T> {
    fn info(&self) -> Info {
        if self.has_right_most_space() {
            Info::RightMostRoom
        } else if self.is_full() {
            Info::Full
        } else {
            Info::RoomInSelf
        }
    }
    fn has_right_most_space(&self) -> bool {
        match self.view() {
            Values(values) => values.len() < SLOTS,
            Links(links) => {
                let last = links
                    .last()
                    .expect("links should have at least one child")
                    .as_ref();
                last.has_right_most_space()
            }
        }
    }

    fn is_full(&self) -> bool {
        match self.view() {
            Values(values) => values.len() >= SLOTS,
            Links(links) => {
                if links.len() >= SLOTS {
                    false
                } else {
                    let last_link = self.last_link().unwrap();
                    last_link.is_full()
                }
            }
        }
    }

    fn links(&self) -> Option<&Vec<Link<T>>> {
        match self {
            Node::Leaf { .. } => None,
            Node::Internal { links, .. } => Some(links),
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

    fn values(&self) -> Option<&Vec<Arc<T>>> {
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
    fn mask(&self, gid: usize) -> usize {
        let mask = (MASK << self.depth());
        gid & mask
    }

    fn update_leaf(&self, gid: usize, value: T) -> Result<Node<T>, ()> {
        let index = self.mask(gid);
        match self {
            Node::Leaf { values } => {
                let mut values = values.clone();
                values[index] = Arc::new(value);
                Ok(Leaf { values })
            }
            Node::Internal { .. } => Err(Issue::WrongNodeOperation),
        }
    }

    fn push_leaf(&self, value: T) -> Result<Node<T>, T> {
        let values = self.values().ok_or(Issue::WrongNodeOperation)?;
        if values.len() >= SLOTS {
            return Err(Issue::LeafFull(value));
        }
        let mut values = values.clone();
        values.push(Arc::new(value));
        Ok(Leaf { values })
    }

    fn anchor(&self, value: T) -> Self {
        match self {
            Node::Leaf { .. } => Leaf {
                values: vec![Arc::new(value)],
            },
            Node::Internal { depth, links } => {
                if *depth == 1 {
                    Internal {
                        depth: *depth,
                        links: vec![Arc::new(Leaf { values: vec![] }.anchor(value))],
                    }
                } else {
                    Internal {
                        depth: *depth,
                        links: vec![Arc::new(Internal {
                            depth: depth - 1,
                            links: vec![],
                        })],
                    }
                }
            }
        }
    }

    fn push(&self, value: T) -> Self {
        match self.info() {
            Info::RightMostRoom => match self.view() {
                Values(vs) => {
                    let mut vs = vs.clone();
                    vs.push(Arc::new(value));
                    return Leaf { values: vs };
                }
                Links(ls) => {
                    let last = ls.last().unwrap().as_ref();
                    let node = last.push(value);
                    let mut ls = ls.clone();
                    let last = ls.last_mut().unwrap();
                    *last = Arc::new(node);
                    return Internal {
                        depth: self.depth(),
                        links: ls,
                    };
                }
            },
            Info::RoomInSelf => {
                if let Some(links) = self.links() {
                    // construct chain
                }
                panic!()
            }
            Info::Full => match self {
                Leaf { values: vs } => {
                    let mut links = vec![
                        Arc::new(Leaf { values: vs.clone() }),
                        Arc::new(Leaf {
                            values: vec![Arc::new(value)],
                        }),
                    ];
                    Internal { depth: 1, links }
                }
                Internal { depth, links: ls } => {
                    let mut links = vec![
                        Arc::new(Internal {
                            depth: self.depth(),
                            links: ls.clone(),
                        }),
                        Arc::new(
                            Internal {
                                depth: self.depth(),
                                links: vec![],
                            }
                            .anchor(value),
                        ),
                    ];
                    Internal {
                        depth: depth+1,
                        links,
                    }
                }
            },
        }
    }
}

impl<T: fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.view() {
            Values(values) => {
                let end = values.len();
                for (i, value) in values {
                    write!(f, "{}", value)?;
                    if i + 1 != end {
                        write!(f, ", ")?;
                    }
                }
            }
            Links(links) => {
                let end = values.len();
                for (i, link) in links {
                    write!(f, "{}", value)?;
                    if i + 1 != end {
                        write!(f, ", ")?;
                    }
                }
            }
        }
        Ok(())
    }
}

type Link<T> = Arc<Node<T>>;

struct Vector<T> {
    length: usize,
    tail: Arc<Node<T>>,
    root: Arc<Node<T>>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn update_leaf() -> Result<(), i32> {
        let mut leaf1 = Leaf { values: vec![] };
        for value in 0..10 {
            leaf1 = leaf1.push(value);
        }

        println!("{:?}", leaf1.has_right_most_space());
        println!("{:?}", leaf1.is_full());
        println!("{:?}", leaf1);
        Ok(())
    }
}
