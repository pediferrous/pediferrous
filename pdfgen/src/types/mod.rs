use std::io::{Error, Write};

pub mod name;

pub trait WriteDictValue {
    fn write(&self, writer: &mut impl Write) -> Result<usize, Error>;
}
