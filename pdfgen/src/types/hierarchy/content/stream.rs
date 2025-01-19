use std::io::{Error, Write};

use pdfgen_macros::const_names;

use crate::types::{
    constants,
    hierarchy::primitives::{name::Name, obj_id::ObjId},
};

/// A stream object, like a string object, is a sequence of bytes that may be of unlimited length.
/// Streams should be used to represent objects with potentially large amounts of data, such as
/// images and page descriptions.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Stream {
    /// Object ID of this `Stream`.
    pub(crate) id: ObjId,

    // NOTE: Stream dictionaries have more entries such as filter, decode parameters etc. For now,
    //       we only need the required dictionary entry 'Length', implicitly available in `Vec`
    //       implementation.
    // TODO: Implement full support for stream dictionary.
    /// Bytes contained in this `Stream` object.
    inner: Vec<u8>,
}

impl Stream {
    const START_STREAM: &[u8] = b"stream";
    const END_STREAM: &[u8] = b"endstream";
    const_names!(LENGTH);

    /// Creates a new empty `Stream`, containing no bytes and with length 0.
    pub fn new(id: ObjId) -> Self {
        Self {
            id,
            inner: Vec::default(),
        }
    }

    /// Creates a new `Stream` with given bytes as the stream's bytes.
    pub fn with_bytes(id: ObjId, bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            id,
            inner: bytes.into(),
        }
    }

    /// Writes (aditional) bytes into this `Stream`, updating it's length.
    pub fn push_bytes(&mut self, bytes: &[u8]) {
        self.inner
            .write_all(bytes)
            .expect("Writing to Vec should never fail.");
    }

    /// Write the stream object into the given implementor of [`Write`] trait, with dictionary
    /// containing only the required `Length` field.
    #[inline(always)]
    pub fn write(&self, writer: &mut dyn Write) -> Result<usize, Error> {
        self.write_with_dict(writer, |_| Ok(0))
    }

    /// Write the stream object into the given implementor of [`Write`] trait, with function that
    /// writes dictionary fields additional to the `Length` field.
    pub fn write_with_dict<F>(&self, writer: &mut dyn Write, write_dict: F) -> Result<usize, Error>
    where
        F: FnOnce(&mut dyn Write) -> Result<usize, Error>,
    {
        let mut written = pdfgen_macros::write_chain! {
            // BEGIN_DICTIONARY:
            writer.write(b"<< "),
            // write the additional dictionary fields
            write_dict(writer),

            // write the length
            Self::LENGTH.write(writer),
            writer.write(self.inner.len().to_string().as_bytes()),
            writer.write(b" >>"),
            writer.write(constants::NL_MARKER),
            // END_DICTIONARY

            // stream
            writer.write(Self::START_STREAM),
            writer.write(constants::NL_MARKER),
        };

        writer.write_all(&self.inner)?;
        written += self.inner.len();

        written += pdfgen_macros::write_chain! {
            writer.write(constants::NL_MARKER),
            writer.write(Self::END_STREAM),
        };

        Ok(written)
    }

    /// Returns `true` if no bytes were written to this [`Stream`].
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::types::hierarchy::primitives::obj_id::IdManager;

    use super::Stream;

    #[test]
    fn basic_stream() {
        let bytes = String::from("This is the content of a stream.");

        let mut id_manager = IdManager::default();
        let stream = Stream::with_bytes(id_manager.create_id(), bytes);

        let mut writer = Vec::default();
        stream.write(&mut writer).unwrap();
        let output = String::from_utf8_lossy(&writer);

        insta::assert_snapshot!(output, @r"
        << /Length 32 >>
        stream
        This is the content of a stream.
        endstream
        ");
    }
}
