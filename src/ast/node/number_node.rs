#[derive(Debug, Clone)]
pub struct NumberNode(pub f64);

impl NumberNode {
    pub fn value(&self) -> f64 {
        self.0
    }
}
