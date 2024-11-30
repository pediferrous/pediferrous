use std::io::{Error, Write};

pub mod hierarchy;
pub use hierarchy::page;

pub trait WriteDictValue {
    fn write(&self, writer: &mut impl Write) -> Result<usize, Error>;
}

macro_rules! write_chain {
    ($($expression:expr),* $(,)?) => {{
        let mut written = 0;
        $(
            written += $expression?;
        )*
        written
    }}
}

pub(crate) use write_chain;
