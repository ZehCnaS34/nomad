use core::cell::RefCell;

#[derive(Debug)]
struct State {
    scan_error: bool,
    parse_error: bool,
    errors: Vec<String>,
}

#[derive(Debug)]
pub struct Context {
    state: RefCell<State>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            state: RefCell::new(State {
                scan_error: false,
                parse_error: false,
                errors: vec![],
            }),
        }
    }

    pub fn is_ok(&self) -> bool {
        let state = self.state.borrow();
        !state.scan_error && !state.parse_error
    }

    pub fn post_error<T: Into<String>>(&self, message: T) {
        let mut state = self.state.borrow_mut();
        state.scan_error = true;
        state.errors.push(message.into())
    }
}
