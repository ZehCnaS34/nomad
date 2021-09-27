use std::borrow::BorrowMut;
use std::fmt;
use std::fmt::Formatter;
use std::sync::Arc;

// use Node::*;
// const BITS: usize = 2;
// const SLOTS: usize = BITS.pow(2);
// const MASK: usize = (1 << BITS) - 1;
//
// enum Operation {
//     InvalidNode(Leaf),
//     InternalFull(Leaf)
// }
//
// #[derive(Clone, Debug)]
// enum NodeAction {
//     TraverseRight,
//     GenInternal,
//     GenRoot,
// }
//
// // reference: https://hypirion.com/musings/understanding-persistent-vector-pt-1
//
// #[derive(Clone, Debug)]
// struct Vector {
//     length: usize,
//     root: Arc<Node>,
//     tail: Arc<Node>,
// }
//
// impl Vector {
//     fn new() -> Vector {
//         Vector {
//             length: 0,
//             root: Arc::new(I(Internal::default())),
//             tail: Arc::new(L(Leaf::default())),
//         }
//     }
//     fn tail_offset(&self) -> usize {
//         self.length - self.tail.len()
//     }
//
//     fn tree_index(&self, index: usize) -> Option<&i32> {
//         todo!()
//     }
//
//     fn tree_update(&self, index: usize, value: i32) -> Vector {
//         todo!()
//     }
//
//     fn get(&self, index: usize) -> Option<&i32> {
//         if index < self.tail_offset() {
//             self.tree_index(index)
//         } else {
//             let leaf = self.tail.as_leaf().expect("Tail should be leaf");
//             leaf.data.get(index - self.tail_offset())
//         }
//     }
//
//     fn is_tail_full(&self) -> bool {
//         self.tail.len() == SLOTS
//     }
//
//     fn conj(&self, value: i32) -> Vector {
//         if self.is_tail_full() {
//             let new_tail = {
//                 let mut tail = Leaf::default();
//                 tail.data.push(value);
//                 tail
//             };
//             let mut vector = self.clone();
//             vector.length += 1;
//             if vector.root.is_full_tree() {
//                 let previous_root = vector.root.clone();
//                 let previous_depth = previous_root.depth();
//                 let mut new_root = Internal::default();
//                 new_root.depth = previous_root.depth() + 1;
//                 new_root.links.push(previous_root);
//
//                 let mut low_inter = Internal::default();
//                 low_inter.links.push(self.tail.clone());
//                 for depth in 2..previous_depth {
//                     let mut high_inter = Internal::default();
//                     high_inter.links.push(Arc::new(I(low_inter)));
//                     low_inter = high_inter;
//                 }
//                 new_root.links.push(Arc::new(I(low_inter)));
//
//                 vector.tail = Arc::new(L(new_tail));
//                 vector.root = Arc::new(I(new_root));
//                 return vector;
//             } else {
//                 let mut internal = vector.root.as_ref().clone().take_internal().unwrap();
//
//
//                 // let mut node = vector.root.as_ref().clone();
//             }
//             todo!()
//         } else {
//             let mut vector = self.clone();
//             let mut tail = vector.tail.as_ref().clone();
//             {
//                 let mut tail = tail.mut_leaf().expect("Tail should always be a leaf.");
//                 tail.data.push(value);
//             }
//             vector.length += 1;
//             vector.tail = Arc::new(tail);
//             vector
//         }
//     }
// }
//
// #[derive(Debug, Clone)]
// struct Internal {
//     depth: usize,
//     links: Vec<Link>,
// }
//
//
// impl Default for Internal {
//     fn default() -> Self {
//         Internal { depth: 1, links: Vec::with_capacity(SLOTS) }
//     }
// }
//
// type Link = Arc<Node>;
// #[derive(Debug, Clone)]
// enum Node {
//     I(Internal),
//     L(Leaf),
// }
//
// impl Node {
//     fn put_leaf(&self, leaf: Leaf) -> Result<Node, Operation> {
//         if self.is_leaf() {
//             Err(Operation::InvalidNode(leaf))
//         } else if if self.is_full_tree() {
//
//             let mut new_root = Internal::default();
//             todo!()
//         } else {
//             todo!()
//         }
//     }
//
//     fn depth (&self) -> usize {
//         match self {
//             I(i) => i.depth,
//             L(l) => 0
//         }
//     }
//     fn is_full_tree(&self) -> bool {
//         match self {
//             I(internal) => {
//                 if internal.links.len() < SLOTS {
//                     return false
//                 } else if let Some(link) = internal.links.last() {
//                     link.is_full_tree()
//                 } else {
//                     unreachable!()
//                 }
//             }
//             L(leaf) => {
//                 leaf.data.len() >= SLOTS
//             }
//         }
//     }
//
//     fn mut_internal(&mut self) -> Option<&mut Internal> {
//         match self {
//             I(i) => Some(i),
//             L(..) => none
//         }
//     }
//
//     fn mut_leaf(&mut self) -> Option<&mut Leaf> {
//         match self {
//             I(_) => None,
//             L(leaf) => Some(leaf)
//         }
//     }
//
//     fn is_leaf(&self) -> bool {
//         self.as_leaf().is_some()
//     }
//
//     fn as_leaf(&self) -> Option<&Leaf> {
//         match self {
//             I(_) => None,
//             L(leaf) => Some(leaf)
//         }
//     }
//
//     fn as_internal (&self) -> Option<&Internal> {
//         match self {
//             I(internal) => Some(internal),
//             L(_) => None
//         }
//     }
//
//     fn take_internal(self) -> Option<Internal> {
//         match self {
//             I(internal) => Some(internal),
//             L(_) => None
//         }
//     }
// }
//
// #[derive(Debug, Clone)]
// struct Leaf {
//     data: Vec<i32>,
// }
//
// impl Default for Leaf {
//     fn default() -> Self {
//         Leaf { data: Vec::with_capacity(SLOTS) }
//     }
// }
//
//
// impl Node {
//     fn len(&self) -> usize {
//         match self {
//             I(i) => i.links.len(),
//             L(l) => l.data.len()
//         }
//     }
// }
//
// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn tail_update() {
//         let v = Vector::new()
//             .conj(1)
//             .conj(2)
//             .conj(3)
//             .conj(4)
//         ;
//         println!("{:?}", v);
//     }
//
//     #[test]
//     fn tail_set_overflow() {
//         let v = Vector::new()
//             .conj(1)
//             .conj(2)
//             .conj(3)
//             .conj(4)
//             ;
//         println!("{:?}", v);
//         let v2 = v.conj(5);
//         println!("{:?}", v2);
//     }
//
// }
