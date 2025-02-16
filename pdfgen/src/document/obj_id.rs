//! Implementation of PDF object reference.

use std::io::{Error, Write};

/// Any object in a PDF file may be labelled as an indirect object. This gives the object a unique
/// object identifier by which other objects can refer to it. The object may be referred to from
/// elsewhere in the file by an indirect reference. Such indirect references shall consist of the
/// object number, the generation number, and the keyword R (with whitespace separating each part).
///
/// Example: `4 0 R`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObjId {
    /// Identifier of referenced object.
    id: u64,
}

impl ObjId {
    /// Marker indicating start of an object section
    const START_OBJ_MARKER: &[u8] = b"obj";

    /// Write the encoded PDF object reference into the provided implementor of [`Write`].
    pub fn write_ref(&self, writer: &mut dyn Write) -> Result<usize, Error> {
        Ok(pdfgen_macros::write_chain! {
            writer.write(self.id.to_string().as_bytes()),
            // NOTE: generation is always 0 because we are genereting new PDFs and don't support
            //       updating existing PDFs
            writer.write(b" 0 R"),
        })
    }

    /// Write the encoded PDF object id into the provided implementor of [`Write`].
    pub fn write_def(&self, writer: &mut dyn Write) -> Result<usize, Error> {
        Ok(pdfgen_macros::write_chain! {
            writer.write(self.id.to_string().as_bytes()),
            // NOTE: generation is always 0 because we are genereting new PDFs and don't support
            //       updating existing PDFs
            writer.write(b" 0 "),
            writer.write(Self::START_OBJ_MARKER),
        })
    }
}

pub(crate) struct IdManager {
    curr: u64,
}

impl IdManager {
    pub(crate) fn new() -> Self {
        Self { curr: 1 }
    }

    /// Creates a clone of this [`IdManager`]. Take great care when doing this, otherwise the
    /// document might get into an invalid state.
    pub(in crate::document) fn clone(&self) -> Self {
        Self { curr: self.curr }
    }

    pub fn create_id(&mut self) -> ObjId {
        let inner_id = self.curr;
        self.curr += 1;
        ObjId { id: inner_id }
    }
}
