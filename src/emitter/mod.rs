use crate::prelude::*;

pub trait Emitter {
    fn emit(&self) -> Result<String>;
}

