pub type Id = usize;
pub type Ids = Vec<Id>;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Tag {
    // Throw away
    Noop,

    // Base case
    Nil(Id),
    Boolean(Id),
    Number(Id),
    String(Id),
    Symbol(Id),
    Quote(Id),
    Meta(Id),
    Decorator(Id),

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
    Macro(Id),
    QuasiQuote(Id),

    // Data
    Vector(Id),

    // Entry
    Program(Id),
}

pub struct TagIter<'a> {
    current: usize,
    tags: &'a [Tag],
}

pub trait Partition {
    type Item;
    fn take_1(self) -> Option<(Self::Item, Vec<Self::Item>)>;
    fn take_2(self) -> Option<(Self::Item, Self::Item, Vec<Self::Item>)>;
    fn take_3(self) -> Option<(Self::Item, Self::Item, Self::Item, Vec<Self::Item>)>;
    fn take_4(
        self,
    ) -> Option<(
        Self::Item,
        Self::Item,
        Self::Item,
        Self::Item,
        Vec<Self::Item>,
    )>;
}

impl Partition for Vec<Tag> {
    type Item = Tag;
    fn take_1(mut self) -> Option<(Tag, Vec<Tag>)> {
        let mut tags = self.into_iter();
        let one = tags.next()?;
        Some((one, tags.collect()))
    }
    fn take_2(mut self) -> Option<(Tag, Tag, Vec<Tag>)> {
        let mut tags = self.into_iter();
        let one = tags.next()?;
        let two = tags.next()?;
        Some((one, two, tags.collect()))
    }
    fn take_3(mut self) -> Option<(Tag, Tag, Tag, Vec<Tag>)> {
        let mut tags = self.into_iter();
        let one = tags.next()?;
        let two = tags.next()?;
        let three = tags.next()?;
        Some((one, two, three, tags.collect()))
    }
    fn take_4(mut self) -> Option<(Tag, Tag, Tag, Tag, Vec<Tag>)> {
        let mut tags = self.into_iter();
        let one = tags.next()?;
        let two = tags.next()?;
        let three = tags.next()?;
        let four = tags.next()?;
        Some((one, two, three, four, tags.collect()))
    }
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

    pub fn on_symbol(&self) -> Option<&Tag> {
        match self {
            Tag::Symbol(_) => Some(self),
            _ => None,
        }
    }

    pub fn take_symbol(self) -> Option<Self> {
        match self {
            Tag::Symbol(_) => Some(self),
            _ => None,
        }
    }

    pub fn is_symbol(&self) -> bool {
        match self {
            Tag::Symbol(_) => true,
            _ => false,
        }
    }

    pub fn take_vector(self) -> Option<Self> {
        match self {
            Tag::Vector(_) => Some(self),
            _ => None,
        }
    }

    pub fn is_atom(&self) -> bool {
        match self {
            Tag::Nil(_) => true,
            Tag::Boolean(_) => true,
            Tag::Number(_) => true,
            Tag::String(_) => true,
            Tag::Symbol(_) => true,
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
