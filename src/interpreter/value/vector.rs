use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard};
use Node::*;
use std::fmt::Formatter;
use std::borrow::BorrowMut;

const N: usize = 3;

#[derive(Debug, Clone)]
struct LeafNode {
    slots: Vec<i32>,
}


impl Default for LeafNode {
    fn default() -> Self {
        LeafNode {
            slots: Vec::with_capacity(N),
        }
    }
}

#[derive(Debug, Clone)]
struct JunctionNode {
    depth: usize,
    links: Vec<Arc<Node>>,
}



impl Default for JunctionNode {
    fn default() -> Self {
        JunctionNode {
            depth: 0,
            links: Vec::with_capacity(N),
        }
    }
}

impl JunctionNode {
    fn map_last<F>(&self, f: F) -> Option<JunctionNode>
    where
        F: Fn(Arc<Node>) -> Arc<Node>
    {
        let node = f(self.links.last().cloned()?);
        let mut junction = self.clone();
        let index = junction.links.len() - 1;
        let last = junction.links.get_mut(index)?;
        *last = node;
        Some(junction)
    }
}


#[derive(Debug, Clone)]
enum Node {
    Leaf(LeafNode),
    Junction(JunctionNode),
}


impl Node {
    fn is_locally_full(&self) -> bool {
        match self {
            Leaf(..) => true,
            Junction(node) => node.links.len() == N,
        }
    }

    fn new() -> Node {
        Leaf(LeafNode::default())
    }

    fn as_junction(&self) -> Option<&JunctionNode> {
        match self {
            Leaf(..) => None,
            Junction(node) => Some(node),
        }
    }

    fn to_junction(self) -> Option<JunctionNode> {
        match self {
            Leaf(..) => None,
            Junction(node) => Some(node),
        }
    }

    fn last_link(&self) -> Option<Arc<Node>> {
        self.as_junction()
            .and_then(|junction| junction.links.last().cloned())
    }

    fn set_last(&mut self, node: Node) {
        self.last_link().replace(Arc::new(node));
    }

    fn is_full(&self) -> bool {
        match self {
            Leaf(node) => node.is_full(),
            Junction(node) => node.is_full()
        }
    }

    fn conj(self: &Arc<Node>, value: i32) -> Arc<Node> {
        Arc::new(match self.as_ref() {
            Leaf(node) => {
                if node.is_full() {
                    println!("leaf::full {}", node);
                    let mut junction = JunctionNode::default();
                    junction.links.push(self.clone());
                    let mut leaf = LeafNode::default();
                    leaf.slots.push(value);
                    junction.links.push(Arc::new(Leaf(leaf)));
                    Junction(junction)
                } else {
                    println!("leaf::room {}", node);
                    // clone and push
                    let mut node = node.clone();
                    node.slots.push(value);
                    Leaf(node)
                }
            }
            Junction(node) => if node.is_full() {
                println!("junction::full {}", node);
                let mut leaf = LeafNode::default();
                leaf.slots.push(value);

                let mut child = Leaf(leaf);
                for depth in 0..node.depth+1 {
                    let mut new_root = JunctionNode::default();
                    new_root.depth = depth;
                    new_root.links.push(Arc::new(child));
                    child = Junction(new_root);
                }

                let mut root = JunctionNode::default();
                root.depth = node.depth + 1;
                root.links.push(self.clone());
                root.links.push(Arc::new(child));
                Junction(root)
            } else {
                println!("junction::room {}", node);
                Junction(
                    node.map_last(|node| {
                        node.conj(value)
                    }).unwrap()
                )
            },
        })
    }

}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn conj() {
        let mut node = Arc::new(Node::new());
        for value in 0..10 {
            println!();
            println!();
            println!("{}", node);
            let after = node.conj(value);
            println!("{}", after);
            println!();
            println!();
            println!();
            node = after;
        }
    }
}

impl fmt::Display for LeafNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let end = self.slots.len();
        write!(f, "[")?;
        for (i, value) in self.slots.iter().enumerate() {
            write!(f, "{}", value)?;
            if i + 1 != end {
                write!(f, ",")?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Junction(node) => write!(f, "{}", node),
            Leaf(node) => write!(f, "{}", node),
        }
    }
}

impl fmt::Display for JunctionNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let end = self.links.len() -1;
        write!(f, "(")?;
        for (i, value) in self.links.iter().enumerate() {
            write!(f, "{}", value)?;
            if i != end {
                write!(f, " ")?;
            }
        }
        write!(f, ")")?;
        Ok(())
    }
}

trait Info {
    fn info(&self) -> NodeInfo;
}

#[derive(Debug)]
enum NodeInfo {
    Empty,
    Room,
    Spill,
    Full,
}

impl Info for LeafNode {
    fn info(&self) -> NodeInfo {
        match self.slots.len() {
            0 => NodeInfo::Empty,
            n if n == N => NodeInfo::Full,
            n => NodeInfo::Room,
        }
    }
}

impl Info for JunctionNode {
    fn info(&self) -> NodeInfo {
        match self.links.len() {
            0 => NodeInfo::Empty,
            n if n == N => match &self.links[n-1].info() {
                NodeInfo::Empty => NodeInfo::Room,
                NodeInfo::Room => NodeInfo::Room,
                NodeInfo::Full => NodeInfo::Full,
                NodeInfo::Spill => unreachable!(),
            }
            n => match &self.links[n-1].info() {
                NodeInfo::Empty => NodeInfo::Room,
                NodeInfo::Room => NodeInfo::Room,
                NodeInfo::Full => NodeInfo::Spill,
                NodeInfo::Spill => NodeInfo::Room,
            }
        }
    }
}

impl Info for Node {
    fn info(&self) -> NodeInfo {
        match self {
            Leaf(node) => node.info(),
            Junction(node) => node.info(),
        }
    }
}
