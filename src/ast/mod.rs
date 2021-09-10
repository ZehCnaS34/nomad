pub mod node;
pub mod parser;
pub mod scanner;

pub struct Limits {
    pub function_call: usize,
    pub program: usize,
    pub while_body: usize,
}

pub const CHILD_LIMIT: Limits = Limits {
    function_call: 8,
    program: 20,
    while_body: 20,
};

#[macro_export]
macro_rules! copy {
    ( $source:ident, $offset:literal, $size:expr ) => {{
        let mut body = [Tag::Noop; $size];
        let mut i = 0;
        for tag in &$source[$offset..$size] {
            body[i] = *tag;
            i += 1;
        }
        body
    }};
}
