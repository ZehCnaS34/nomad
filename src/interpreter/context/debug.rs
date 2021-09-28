use crate::result::RuntimeResult as Result;

pub trait Dump {
    fn dump(&self) -> Result<()>;
}
