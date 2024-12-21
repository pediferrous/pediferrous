use crate::types::hierarchy::content::stream::Stream;

use super::{obj_id::ObjId, object::Object};

/// Represents a PDF String with UTF-8 encoding.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PdfString {
    /// Inner [`Stream`] is the object that actually stores the bytes of a `PdfString`.
    stream: Stream,
}

impl PdfString {
    /// Creates a new `PdfString` with the given value that can be converted to UTF-8 encoded
    /// [`String`].
    pub fn from(id: ObjId, string: impl Into<String>) -> Self {
        // NOTE: For text strings encoded in UTF-8, the first three bytes shall be 239 followed by
        //       187, followed by 191.
        let mut bytes = vec![b'(', 239, 187, 191];

        bytes.extend(string.into().into_bytes());
        bytes.push(b')');

        Self {
            stream: Stream::with_bytes(id, bytes),
        }
    }
}

impl Object for PdfString {
    fn write(&self, writer: &mut impl std::io::Write) -> Result<usize, std::io::Error> {
        self.stream.write(writer)
    }
}
