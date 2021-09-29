use crate::result::Result;

pub trait Dump {
    fn dump(&self) -> Result<()>;
}
