use Issue::*;
use std::ops::{IndexMut, Index};

#[derive(Clone, Debug)]
enum Issue<T> {
    Full(T),
    InvalidIndex(T),
}

#[derive(Clone, Debug)]
struct Leaf<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Leaf<T> {
    fn is_overflow(&self, index: usize) -> bool {
        index >= self.size || index >= self.data.len()
    }
    fn is_full(&self) -> bool {
        self.size == self.data.len()
    }

    fn new() -> Leaf<T> {
        Leaf {
            size: 4 as usize,
            data: Vec::with_capacity(4),
        }
    }

    fn push(&self, value: T) -> Result<Leaf<T>, Issue<T>>
    where
        T: Clone,
    {
        if self.is_full() {
            return Err(Full(value));
        } else {
            let mut leaf = self.clone();
            leaf.data.push(value);
            return Ok(leaf);
        }
    }

    fn replace(&self, index: usize, value: T) -> Result<Leaf<T>, Issue<T>> 
    where T: Clone{
        if self.is_overflow(index) {
            Err(InvalidIndex(value))
        } else {
            let mut leaf = self.clone();
            leaf.data.remove(index);
            leaf.data.insert(index, value);
            Ok(leaf)
        }
    }

    fn remove(&self, index: usize) -> Result<Leaf<T>, Issue<()>>
    where T: Clone {
        if self.is_overflow(index) {
            Err(InvalidIndex(()))
        } else {
            let mut leaf = self.clone();
            leaf.data.remove(index);
            Ok(leaf)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn leaf_push() -> Result<(), Issue<i32>> {
        let base: Leaf<i32> = Leaf::new().push(1)?.push(2)?;
        let no_one = base.remove(0);
        println!("{:?}", base);
        println!("{:?}", no_one);
        Ok(())

    }
}
