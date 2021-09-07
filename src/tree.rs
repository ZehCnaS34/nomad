use std::rc::Rc;

pub enum FingerTree<T> {
    Empty,
    Single(T),
    Deep(Digit<T>, Rc<FingerTree<Node<T>>>, Digit<T>),
}

enum Node<T> {
    Node2(T, T),
    Node3(T, T),
}

enum Digit<T> {
    One(T),
    Two(T, T),
    Three(T, T, T),
    Four(T, T, T, T),
}

impl<T> FingerTree<T> {
    pub fn new() -> FingerTree<T> {
        FingerTree::Empty
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn simple_tree() {}
}
