pub trait Operation {
    fn is_truthy(&self) -> bool;

    fn is_falsy(&self) -> bool {
        !self.is_truthy()
    }

    fn eq(&self, other: &Self) -> bool;

    fn neq(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}