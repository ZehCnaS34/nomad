pub trait Matcher {
    type Item;
    fn check_not(&self, offset: usize, value: Self::Item) -> bool;
    fn check(&self, offset: usize, value: Self::Item) -> bool;
    fn test<F>(&self, offset: usize, tester: F) -> bool
    where
        F: Fn(Self::Item) -> bool;
    fn check_next(&self, offset: usize, value: Self::Item) -> bool;
    fn test_next<F>(&self, offset: usize, tester: F) -> bool
    where
        F: Fn(Self::Item) -> bool;
}
