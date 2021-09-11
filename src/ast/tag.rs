pub type Id = usize;
pub type Ids = Vec<Id>;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Tag {
    // Throw away
    Noop,

    // Base case
    Atom(Id),

    // Defining bindings
    Definition(Id),

    // Function
    Call(Id),
    Function(Id),

    // Control flow
    Do(Id),
    If(Id),
    While(Id),
    Let(Id),
    Loop(Id),
    Recur(Id),

    // Data
    Vector(Id),

    // Entry
    Program(Id),
}

pub struct TagIter<'a> {
    current: usize,
    tags: &'a [Tag],
}

#[macro_export]
macro_rules! take_tags {
    ( $tags:ident, $amount:literal ) => {};
}

impl Tag {
    pub fn tags(tags: &[Tag]) -> TagIter {
        TagIter { tags, current: 0 }
    }

    pub fn len(tags: &[Tag]) -> usize {
        let mut i = 0;
        for _ in Tag::tags(tags) {
            i += 1;
        }
        return i;
    }

    pub fn is_vector(&self) -> bool {
        match self {
            Tag::Vector(_) => true,
            _ => false,
        }
    }

    pub fn is_atom(&self) -> bool {
        match self {
            Tag::Atom(_) => true,
            _ => false,
        }
    }
}

impl<'a> Iterator for TagIter<'a> {
    type Item = Tag;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.tags.len() {
            return None;
        }
        let tag = self.tags[self.current];
        if tag == Tag::Noop {
            return None;
        }
        self.current += 1;
        Some(tag)
    }
}
