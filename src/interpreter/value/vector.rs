use std::borrow::BorrowMut;
use std::fmt;
use std::fmt::Formatter;
use std::mem;
use std::ops::Deref;
use std::sync::{Arc, Mutex, MutexGuard};
use Node::*;
use Insert::*;

const N: usize = 3;

enum Insert<P, D> {
    Put(P),
    Drop(D),
}


#[derive(Debug, Clone)]
struct Leaf<T> {
    slots: [Option<T>; N],
}

impl<T> Leaf<T> {
    fn push(self) -> Insert<Leaf<T>, T> {
    }
}

#[derive(Debug, Clone)]
struct Internal<T> {
    slots: [Option<Arc<Node<T>>>; N],
}

#[derive(Debug, Clone)]
enum Node<T> {
    L(Leaf<T>),
    I(Internal<T>),
}

impl<T: Clone + fmt::Debug> Node<T> {
    fn push_value(&self, value: T) -> Insert<Node<T>, T>
    where
        T: fmt::Debug + fmt::Debug,
    {
        match self {
            I(..) => Drop(value),
            L(leaf) => {
                let mut leaf = leaf.clone();
                Drop(value)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn leaf() {
        let leaf = Node::<i32>::L(Leaf { slots: [None; N] });
        println!("{:#?}", leaf);
        leaf.push_value(23);
    }
}
