//! Implementation of Resources Dictionary data type.

use std::io::{Error, Write};

use crate::types;

/// Represents a single entry in the [`Resources`] dictionary.
#[derive(Debug, Clone)]
pub(crate) enum ResourceEntry {}

impl ResourceEntry {
    /// Encode and write this entry into the implementor of [`Write`].
    fn write(&self, _writer: &mut dyn Write) -> Result<usize, Error> {
        Ok(0)
    }
}

/// Resource dictionary enumerates the named resources needed by the operators in the content
/// stream and the names by which they can be referred to.
///
/// This is used in cases, where an operator shall refer to a PDF object that is defined outside
/// the content stream, such as a font dictionary or a stream containing image data. This shall be
/// accomplished by defining such objects as named resources and referring to them by name from
/// within the content stream.
#[derive(Default, Debug, Clone)]
pub struct Resources {
    entries: Vec<ResourceEntry>,
}

impl Resources {
    /// Encode and write this resource dictionary into the provided implementor of [`Write`].
    pub(crate) fn write(&self, writer: &mut impl Write) -> Result<usize, Error> {
        let written = types::write_chain! {
            writer.write(b"<< "),
            self.entries.iter().map(|entry| entry.write(writer)).sum::<Result<usize, _>>(),
            writer.write(b" >>"),
        };

        Ok(written)
    }
}
