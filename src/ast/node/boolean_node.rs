#[derive(Debug, Clone)]
pub struct BooleanNode(pub bool);

impl BooleanNode {
    pub fn value(&self) -> bool {
        self.0
    }
}