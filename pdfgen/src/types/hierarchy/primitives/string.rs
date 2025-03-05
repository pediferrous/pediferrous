use std::io::{Error, Write};

use pdfgen_macros::write_chain;

/// Represents a PDF String with UTF-8 encoding.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PdfString {
    /// Inner [`String`] is the object that actually stores the bytes of a `PdfString`.
    inner: String,
}

impl PdfString {
    /// Creates a new `PdfString` with the given content that can be converted to UTF-8 encoded
    /// [`String`].
    pub fn from(content: impl Into<String>) -> Self {
        Self {
            inner: content.into(),
        }
    }

    /// Expands the `PdfString` with the given content that can be converted to UTF-8 encoded
    /// [`String`].
    pub fn expand(&mut self, content: impl Into<String>) {
        self.inner.push_str(&content.into())
    }

    /// Writes the inner content in the PDF String syntax format to the provided writer.
    pub fn write_content(&self, writer: &mut dyn Write) -> Result<usize, Error> {
        Ok(write_chain! {
            writer.write(b"("),
            writer.write(self.inner.as_bytes()),
            writer.write(b")"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::PdfString;

    #[test]
    fn simple_string() {
        let pdf_string = PdfString::from("This is text.");

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

    #[test]
    fn simple_string_expanded() {
        let mut pdf_string = PdfString::from("This is");
        pdf_string.expand(" an expanded text.");

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
