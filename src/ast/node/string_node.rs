#[derive(Debug, Clone)]
pub struct StringNode {
    literal: String,
}

impl StringNode {
    pub fn from(literal: &str) -> StringNode {
        StringNode {
            literal: String::from(literal),
        }
    }

    pub fn value(&self) -> &str {
        &self.literal[..]
    }
}
