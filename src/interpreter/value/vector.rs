use std::fmt;
use std::fmt::Formatter;
use std::sync::Arc;
use Node::*;
use std::borrow::BorrowMut;

// reference: https://hypirion.com/musings/understanding-persistent-vector-pt-1

const N: usize = 2;

trait Bucket {
    fn has_space(&self) -> bool;
}

#[derive(Debug, Clone)]
struct Vector {
    length: usize,
    root: Arc<Node>,
}

impl Vector {
    fn new() -> Vector {
        Vector {
            length: 0,
            root: Arc::new(Node::new_leaf()),
        }
    }
}

#[derive(Debug, Clone)]
enum Node {
    I(Internal),
    L(Leaf),
}

type Link = Arc<Node>;
#[derive(Debug, Clone)]
struct Internal {
    depth: usize,
    links: Vec<Link>,
}

impl Internal {
    fn get(&self, index: usize) -> Option<Link> {
        let index = index.local(self.depth);
        self.links.get(index).cloned()
    }

    fn last(&self) -> Option<Arc<Node>> {
        self.links.last().map(|link| link.clone())
    }
}

#[derive(Debug, Clone)]
struct Leaf {
    data: Vec<i32>,
}

impl Leaf {
    fn get(&self, index: usize) -> Option<i32> {
        let index = index.local(0);
        self.data.get(index).cloned()
    }
}

#[derive(Debug, Clone)]
enum Info {
    RoomLeaf,
    RoomInternal,
    Full,
}

impl Node {
    fn has_root_space(&self) -> bool {
        match self {
            I(i) => i.links.len() < N,
            L(l) => l.data.len() < N,
        }
    }

    fn has_right_most_space(&self) -> bool {
        match self {
            I(i) => i
                .links
                .last()
                .map(|link| link.as_ref().has_right_most_space())
                .unwrap_or(true),
            L(l) => l.data.len() < N,
        }
    }

    fn new_internal() -> Node {
        I(Internal::default())
    }

    fn new_leaf() -> Node {
        L(Leaf::default())
    }

    fn conj(self: Arc<Node>, value: i32) -> Arc<Node> {
        let node = self.clone();
        match node.as_ref() {
            L(leaf) => {
                if !leaf.has_space() {
                    let mut internal = Internal::default();
                    let mut child = {
                        let mut leaf = Leaf::default();
                        leaf.data.push(value);
                        L(leaf)
                    };
                    internal.links.push(node);
                    internal.links.push(Arc::new(child));
                    Arc::new(I(internal))
                } else {
                    let mut leaf = leaf.clone();
                    leaf.data.push(value);
                    Arc::new(L(leaf))
                }
            }
            I(internal) => {
                if let Some(last_child) = internal.last() {
                    if last_child.has_right_most_space() {
                        let mut internal = internal.clone();
                        let last_child = last_child.conj(value);
                        let omg = internal.links.last_mut().unwrap();
                        *omg = last_child;
                        return Arc::new(I(internal));
                    } else if last_child.has_root_space() {
                        let mut internal = internal.clone();
                        internal.links.push(Arc::new(child));
                        println!("internal = {:?}", internal);
                        Arc::new(I(internal))
                    }
                    todo!()
                } else if internal.depth == 1 {
                    let mut internal = internal.clone();
                    // conj leaf
                    let child = {
                        let mut leaf = Leaf::default();
                        leaf.data.push(value);
                        L(leaf)
                    };
                    internal.links.push(Arc::new(child));
                    return Arc::new(I(internal));
                } else {
                    // insert internal
                    let mut internal = internal.clone();
                    let mut sub_internal = Internal::default();
                    sub_internal.depth = internal.depth - 1;
                    let child_node = Arc::new(I(sub_internal)).conj(value);
                    internal.links.push(child_node);
                    return Arc::new(I(internal));
                }
            }
        }
    }
}

trait NodeIndex {
    fn local(&self, depth: usize) -> usize;
    fn is_over(&self) -> bool;
}

impl NodeIndex for usize {
    fn local(&self, depth: usize) -> usize {
        let mask = (!(usize::MAX << N) << (depth * N));
        let result = (mask & self) >> (depth * N);
        println!("{:08b} {:08b} {:08b}", mask, self, result);
        result
    }

    fn is_over(&self) -> bool {
        *self >= N.pow(2)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn setting() {
        let base = Arc::new(Node::new_leaf())
            .conj(3)
            .conj(4)
            .conj(5)
            .conj(6)
            .conj(7)
        ;
        println!("{}", base);
        // let zer = Node::new_leaf();
        // let one = zer.conj(1);
        // let two = zer.conj(2);
        // println!("{:?}", zer);
        // println!("{:?}", one);
        // println!("{:?}", two);
    }
}

impl Bucket for Node {
    fn has_space(&self) -> bool {
        match self {
            I(node) => node.has_space(),
            L(node) => node.has_space(),
        }
    }
}

impl Bucket for Internal {
    fn has_space(&self) -> bool {
        self.links.len() < N
    }
}

impl Bucket for Leaf {
    fn has_space(&self) -> bool {
        self.data.len() < N
    }
}

impl Default for Internal {
    fn default() -> Internal {
        Internal {
            depth: 1,
            links: Vec::with_capacity(N),
        }
    }
}

impl Default for Leaf {
    fn default() -> Leaf {
        Leaf {
            data: Vec::with_capacity(N),
        }
    }
}

impl fmt::Display for Leaf {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let end = self.data.len();
        for (i, data) in self.data.iter().enumerate() {
            write!(f, "{}", data)?;
            if (i + 1) < end {
                write!(f, ",")?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for Internal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        let end = self.links.len();
        for (i, data) in self.links.iter().enumerate() {
            write!(f, "{}", data)?;
            if i + 1 < end {
                write!(f, ";")?;
            }
        }
        write!(f, ")")?;
        Ok(())
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            I(i) => write!(f, "{}", i),
            L(l) => write!(f, "{}", l),
        }
    }
}
