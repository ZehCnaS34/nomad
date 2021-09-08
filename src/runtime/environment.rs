use crate::ast::node::Node;

pub enum State {
    Running,
}

pub struct Environment {
    state: State,
}

impl Environment {
    fn new() -> Environment {
        Environment {
            state: State::Running,
        }
    }

    fn execute(&mut self, node: Node) {}
}
