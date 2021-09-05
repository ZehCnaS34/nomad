use std::collections::HashMap;

pub type Target = [char; 512];

pub type Label = i32;

pub trait Link {
    fn label(&self) -> Label;
    fn parent(&self) -> Option<Label>;
    fn set_parent(&mut self, parent: Label);
}

#[derive(Debug)]
pub struct Chain<T: Link> {
    nodes: HashMap<Label, T>,
    current: Option<Label>,
}

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

impl<T: Link> Chain<T> {
    pub fn new(start: T) -> Chain<T> {
        let current = start.label();
        let nodes = map! { current => start };
        Chain {
            nodes,
            current: Some(current),
        }
    }

    pub fn iter<'a>(&'a self) -> ChainIter<'a, T> {
        ChainIter {
            label: self.current.clone(),
            chain: self,
        }
    }

    pub fn get_current(&self) -> Option<&T> {
        self.current.and_then(|current| self.nodes.get(&current))
    }

    pub fn get_mut_current(&mut self) -> Option<&mut T> {
        self.current
            .and_then(move |current| self.nodes.get_mut(&current))
    }

    pub fn push(&mut self, n: T) {
        let mut n = n;
        if let Some(parent_node) = self.get_current() {
            n.set_parent(parent_node.label());
        }
        self.current = Some(n.label());
        self.nodes.insert(n.label(), n);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.current
            .and_then(|current| {
                self.nodes.remove(&current).and_then(|child| {
                    child.parent().and_then(|old_label| {
                        self.current = Some(old_label);
                        Some(child)
                    })
                })
            })
            .or(None)
    }
}

pub struct ChainIter<'a, T: Link> {
    chain: &'a Chain<T>,
    label: Option<Label>,
}

impl<'a, T: Link> Iterator for ChainIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(label) = self.label {
            if let Some(output) = self.chain.nodes.get(&label) {
                self.label = output.parent();
                return Some(output);
            }
        }
        None
    }
}
