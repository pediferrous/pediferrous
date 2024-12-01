//! Implementation of Resources Dictionary data type.

use std::io::{Error, Write};

use crate::types;

#[derive(Debug, Clone)]
pub(crate) enum ResourceEntry {}

impl ResourceEntry {
    fn write(&self, _writer: &mut impl Write) -> Result<usize, Error> {
        Ok(0)
    }
}

#[derive(Default, Debug, Clone)]
pub struct Resources {
    entries: Vec<ResourceEntry>,
}

impl Resources {
    pub(crate) fn write(&self, writer: &mut impl Write) -> Result<usize, Error> {
        let written = types::write_chain! {
            writer.write(b"<< "),
            self.entries.iter().map(|entry| entry.write(writer)).sum::<Result<usize, _>>(),
            writer.write(b" >>"),
        };

        Ok(written)
    }
}
