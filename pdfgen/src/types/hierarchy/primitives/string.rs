use std::io::{Error, Write};

use crate::{
    types::{constants, hierarchy::content::stream::Stream},
    ObjId,
};

use super::object::Object;

/// Represents a PDF String with UTF-8 encoding.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PdfString {
    id: ObjId,

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
            id,
            stream: Stream::with_bytes(bytes),
        }
    }
}

impl Object for PdfString {
    fn write_def(&self, writer: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        Ok(pdfgen_macros::write_chain! {
            self.id.write_def(writer),
            writer.write(constants::NL_MARKER),
        })
    }

    fn write_content(&self, writer: &mut dyn Write) -> Result<usize, Error> {
        self.stream.write(writer)
    }
}

#[cfg(test)]
mod tests {
    use crate::{types::hierarchy::primitives::object::Object, IdManager};

    use super::PdfString;

    #[test]
    fn simple_string() {
        let mut id_manager = IdManager::new();
        let pdf_string = PdfString::from(id_manager.create_id(), "This is text.");

        let mut writer = Vec::default();
        pdf_string.write_content(&mut writer).unwrap();
        let output = String::from_utf8(writer).unwrap();

        insta::assert_snapshot!(output, @r"
        << /Length 18 >>
        stream
        (﻿This is text.)
        endstream
        ");
    }
}
