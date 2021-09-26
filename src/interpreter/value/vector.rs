use std::sync::Arc;

// reference: https://hypirion.com/musings/understanding-persistent-vector-pt-1

const N: usize = 4;

struct Node {
    data: Vec<Link>
}

type Link = Option<Arc<i32>>;

impl Node {
    fn new() -> Node {
        Node {
            data: Vec::with_capacity(N),
        }
    }
}