use std::sync::Arc;
use Node::*;

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

    fn conj(&self, value: i32) -> Vector {
        let mut path = Vec::new();
        let mut node = self.root.as_ref();
        // loop {
        //     match node {
        //         L(leaf) => {
        //             if leaf.has_space() {
        //                 let mut new_leaf = leaf.clone();
        //                 new_leaf.data.push(value);
        //                 path.push(L(new_leaf));
        //                 break;
        //             } else {
        //
        //             }
        //         }
        //         I(_) => {}
        //     }
        // }
        todo!()
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
        let index = index.local(depth);
        self.links.get(index).cloned()
    }

    fn last(&self) -> Option<&Node> {
        self.links.last().map(|link| {
            let node = link.as_ref();
            node
        })
    }
}

#[derive(Debug, Clone)]
struct Leaf {
    data: Vec<i32>,
}

impl Leaf {
    fn get(&self, index: usize) -> Option<i32> {
        let index = index.local(depth);
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
    fn has_left_mode_space(&self) -> bool {
        match self {
            I(i) => i
                .links
                .last()
                .map(|link| link.as_ref().has_left_mode_space())
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

    fn conj(self: &Node, value: i32) -> Node {
        let mut node = self.clone();
        match node {
            L(mut node) => {
                if !node.has_space() {
                    panic!("We should never get to this situation");
                } else {
                    node.data.push(value);
                    L(node)
                }
            }
            I(node) => {
                todo!()
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
        let node = Arc::new(Node::new_leaf());
        println!("{:?}", node.info());
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
