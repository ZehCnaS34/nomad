pub type Id = usize;
pub type Ids = Vec<Id>;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum TagKind {
    // Base case
    Nil,
    Boolean,
    Number,
    String,
    Symbol,
    Quote,
    Meta,
    Decorator,

    // Defining bindings
    Definition,

    // Function
    Call,
    Function,

    // Control flow
    Do,
    If,
    While,
    Let,
    Loop,
    Recur,
    Macro,
    QuasiQuote,

    // Data
    Vector,

    // Entry
    Program,
}

impl TagKind {
    pub fn reify(self, id: Id) -> Tag {
        Tag { kind: self, id }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Tag {
    kind: TagKind,
    id: Id,
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

impl<T> Partition for Vec<T> {
    type Item = T;
    fn take_1(mut self) -> Option<(T, Vec<T>)> {
        let mut tags = self.into_iter();
        let one = tags.next()?;
        Some((one, tags.collect()))
    }
    fn take_2(mut self) -> Option<(T, T, Vec<T>)> {
        let mut tags = self.into_iter();
        let one = tags.next()?;
        let two = tags.next()?;
        Some((one, two, tags.collect()))
    }
    fn take_3(mut self) -> Option<(T, T, T, Vec<T>)> {
        let mut tags = self.into_iter();
        let one = tags.next()?;
        let two = tags.next()?;
        let three = tags.next()?;
        Some((one, two, three, tags.collect()))
    }
    fn take_4(mut self) -> Option<(T, T, T, T, Vec<T>)> {
        let mut tags = self.into_iter();
        let one = tags.next()?;
        let two = tags.next()?;
        let three = tags.next()?;
        let four = tags.next()?;
        Some((one, two, three, four, tags.collect()))
    }
}

impl Tag {
    pub fn is_vector(&self) -> bool {
        match self.kind {
            TagKind::Vector => true,
            _ => false,
        }
    }

    pub fn on_symbol(&self) -> Option<&Tag> {
        match self.kind {
            TagKind::Symbol => Some(self),
            _ => None,
        }
    }

    pub fn take_symbol(self) -> Option<Self> {
        match self.kind {
            TagKind::Symbol => Some(self),
            _ => None,
        }
    }

    pub fn is_symbol(&self) -> bool {
        match self.kind {
            TagKind::Symbol => true,
            _ => false,
        }
    }

    pub fn take_vector(self) -> Option<Self> {
        match self.kind {
            TagKind::Vector => Some(self),
            _ => None,
        }
    }

    pub fn is_atom(&self) -> bool {
        match self.kind {
            TagKind::Nil => true,
            TagKind::Boolean => true,
            TagKind::Number => true,
            TagKind::String => true,
            TagKind::Symbol => true,
            _ => false,
        }
    }
}
