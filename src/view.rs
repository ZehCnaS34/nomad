use std::cell::RefCell;

pub trait KindMatcher {
    type Kind;
    fn peek_kind_test(&self, kind: Self::Kind) -> bool;
}

#[derive(Clone, Debug)]
pub struct Cursor {
    pub start: usize,
    pub current: usize,
    pub line: usize,
}

#[derive(Clone, Debug)]
pub struct View<T: Sized + Clone> {
    pub data: Vec<T>,
    pub cursor: RefCell<Cursor>,
}

impl<Type: Sized + Clone> View<Type> {
    pub fn eos(&self) -> bool {
        let cursor = self.cursor.borrow();
        cursor.current >= self.data.len()
    }

    pub fn view(&self) -> Vec<Type> {
        let cursor = self.cursor.borrow();
        let output: Vec<Type> = self
            .data
            .get(cursor.start..cursor.current)
            .map(|items| items.iter().map(|item| item.clone()).collect())
            .unwrap_or(vec![]);
        return output.clone();
    }

    pub fn new<NewType: Sized + Clone>(data: Vec<NewType>) -> View<NewType> {
        View {
            data,
            cursor: RefCell::new(Cursor {
                start: 0,
                current: 0,
                line: 0,
            }),
        }
    }

    pub fn newline(&self) {
        let mut cursor = self.cursor.borrow_mut();
        cursor.line += 1;
    }

    pub fn reset(&self) {
        let mut cursor = self.cursor.borrow_mut();
        cursor.start = cursor.current;
    }

    pub fn peek(&self) -> Option<&Type> {
        let cursor = self.cursor.borrow();
        self.data.get(cursor.current)
    }

    pub fn peek_test<F>(&self, tester: F) -> bool
    where
        F: Fn(&Type) -> bool,
    {
        self.peek().map(tester).unwrap_or(false)
    }

    pub fn peek_next(&self) -> Option<&Type> {
        let cursor = self.cursor.borrow();
        self.data.get(cursor.current + 1)
    }

    pub fn peek_next_test<F>(&self, tester: F) -> bool
    where
        F: Fn(&Type) -> bool,
    {
        self.peek_next().map(tester).unwrap_or(false)
    }

    pub fn advance(&self) -> Option<&Type> {
        let mut cursor = self.cursor.borrow_mut();
        let output = self.data.get(cursor.current);
        cursor.current += if cursor.current < self.data.len() {
            1
        } else {
            0
        };
        output
    }

    pub fn advance_if_true<F>(&self, tester: F) -> bool
    where
        F: Fn(&Type) -> bool,
    {
        return if self.peek_test(tester) {
            self.advance();
            true
        } else {
            false
        };
    }
}
