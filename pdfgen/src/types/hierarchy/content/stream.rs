use std::io::Write;

use crate::types::{
    self, constants,
    hierarchy::primitives::{name::Name, obj_id::ObjId, object::Object},
};

/// A stream object, like a string object, is a sequence of bytes that may be of unlimited length.
/// Streams should be used to represent objects with potentially large amounts of data, such as
/// images and page descriptions.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Stream {
    /// Object ID of this `Stream`.
    id: ObjId,

    // NOTE: Stream dictionaries have more entries such as filter, decode parameters etc. For now,
    //       we only need the required dictionary entry 'Length', implicitly available in `Vec`
    //       implementation.
    // TODO: Implement full support for stream dictionary.
    /// Bytes contained in this `Stream` object.
    inner: Vec<u8>,
}

// TODO: remove this lint as soon as we start using the `Stream`.
#[allow(dead_code)]
impl Stream {
    const START_STREAM: &[u8] = b"stream";
    const END_STREAM: &[u8] = b"endstream";
    const LENGTH: Name = Name::new(b"Length");

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
}

impl Object for Stream {
    fn write(&self, writer: &mut impl std::io::Write) -> Result<usize, std::io::Error> {
        let written = types::write_chain! {
            // obj def
            self.id.write_def(writer),
            writer.write(constants::NL_MARKER),

            // dictionary with length
            writer.write(b"<< "),
            Self::LENGTH.write(writer),
            writer.write(self.inner.len().to_string().as_bytes()),
            writer.write(b" >>"),
            writer.write(constants::NL_MARKER),

            // stream
            writer.write(Self::START_STREAM),
            writer.write(constants::NL_MARKER),
            writer.write(&self.inner),
            writer.write(constants::NL_MARKER),
            writer.write(Self::END_STREAM),
        };

        Ok(written)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::hierarchy::primitives::{obj_id::IdManager, object::Object};

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
        0 0 obj
        << /Length 32 >>
        stream
        This is the content of a stream.
        endstream
        ");
    }
}
