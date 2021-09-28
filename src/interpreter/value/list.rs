use std::borrow::Borrow;
use std::fmt;
use std::fmt::Formatter;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node { value, next: None }
    }
}

type Link<T> = Option<Arc<Node<T>>>;

#[derive(Debug, Clone)]
struct List<T> {
    length: usize,
    head: Link<T>,
}

impl<T> List<T> {
    fn empty() -> List<T> {
        List {
            length: 0,
            head: None,
        }
    }

    fn conj(&self, value: T) -> List<T> {
        let new_head = Node {
            value,
            next: self.head.clone(),
        };
        List {
            length: self.length + 1,
            head: Some(Arc::new(new_head)),
        }
    }

    fn head(&self) -> Option<&T> {
        let head = self.head.as_deref()?;
        Some(head.value.borrow())
    }

    fn tail(&self) -> List<T> {
        List {
            length: if self.length == 0 { 0 } else { self.length - 1 },
            head: { self.head.as_deref().and_then(|head| head.next.clone()) },
        }
    }

    fn iter(&self) -> Iter<T> {
        Iter {
            current: self.head.as_deref(),
        }
    }

    fn rev(&self) -> List<T>
    where
        T: Clone,
    {
        let mut list = List::empty();
        for item in self.iter() {
            list = list.conj(item.clone())
        }
        list
    }

    fn make(&self, node: Node<T>) -> List<T> {
        List {
            length: self.length + 1,
            head: Some(Arc::new(node)),
        }
    }

    fn insert(&self, index: usize, value: T) -> Option<List<T>>
    where
        T: Clone + fmt::Debug,
    {
        println!("INSERT index={:?}, value={:?}", index, value);
        if index == 0 {
            Some(self.conj(value))
        } else {
            let head = self.head()?.clone();
            let rest = self.tail().insert(index - 1, value)?;
            Some(rest.conj(head))
        }
    }
}

impl<T: fmt::Display> fmt::Display for List<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for item in self.iter() {
            write!(f, "{} ", item)?;
        }
        writeln!(f)?;
        Ok(())
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Arc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

struct Iter<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.current?;
        self.current = node.next.as_deref();
        Some(node.value.borrow())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn conj() {
        let mut list = List::empty();
        for value in 0..100 {
            list = list.conj(value);
        }
        for (value, key) in (0..100).rev().zip(list.iter()) {
            assert_eq!(value, *key);
        }
    }
}
