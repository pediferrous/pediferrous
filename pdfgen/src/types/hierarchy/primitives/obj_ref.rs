//! Implementation of PDF object reference.

use std::io::{Error, Write};

use crate::types;

/// Any object in a PDF file may be labelled as an indirect object. This gives the object a unique
/// object identifier by which other objects can refer to it. The object may be referred to from
/// elsewhere in the file by an indirect reference. Such indirect references shall consist of the
/// object number, the generation number, and the keyword R (with whitespace separating each part).
///
/// Example: `4 0 R`
pub struct ObjRef {
    /// Identifier of referenced object.
    id: u64,
}

impl From<u64> for ObjRef {
    fn from(id: u64) -> Self {
        Self { id }
    }
}

impl ObjRef {
    /// Write the encoded PDF object reference into the provided implementor of [`Write`].
    pub fn write(&self, writer: &mut impl Write) -> Result<usize, Error> {
        let written = types::write_chain! {
            writer.write(self.id.to_string().as_bytes()),
            // NOTE: generation is always 0 because we are genereting new PDFs and don't support
            //       updating existing PDFs
            writer.write(b" 0 R"),
        };

        Ok(written)
    }
}
