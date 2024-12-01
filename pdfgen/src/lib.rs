#![forbid(unsafe_code)]

//! `pdfgen` is a low-level library that offers fine-grained control over PDF syntax and
//! PDF file generation.

use std::io::{self, Write};

pub mod types;

/// Comment
pub trait Object {
    /// Comment
    fn write(&self, writer: &mut impl Write) -> Result<usize, io::Error>;
}

/// A wrapper around any type that implements `Write`, adding pdf specific functionality to keep a
/// clear and consistent CrossReferenceTable state
pub struct PdfWriter<W: Write> {
    /// Inner member, representing a type that implements `Write`.
    inner: W,
    /// Current byte offset from the top of document, representing the current position of the `cursor`.
    current_offset: usize,
}

impl<W: Write> PdfWriter<W> {
    /// The PDF file begins with the 5 characters “%PDF–X.X” and byte offsets shall be calculated
    /// from the PERCENT SIGN.
    const PDF_HEADER: &[u8] = b"%PDF-2.0";
    /// The last line of the file shall contain only the end-of-file marker, %%EOF
    const EOF_MARKER: &[u8] = b"%%EOF";

    /// Creates a new `PdfWriter` instance.
    pub fn new(inner: W) -> Self {
        PdfWriter {
            inner,
            current_offset: 0,
        }
    }

    /// Comment
    pub fn write_header(&mut self) -> Result<(), io::Error> {
        // Delegate the actual writing to the inner writer incrementing the current_offset to
        // reflect current 'cursor' position.
        self.current_offset += self.inner.write(Self::PDF_HEADER)?;

        Ok(())
    }

    /// Comment
    pub fn write_object(&mut self, obj: &impl Object) -> Result<(), io::Error> {
        // Delegate the actual writing to the inner writer.
        let written = obj.write(&mut self.inner)?;

        Ok(())
    }

    /// Write the PDF documents EOF.
    pub fn write_eof(&mut self) -> Result<(), io::Error> {
        // Delegate the actual writing to the inner writer.
        self.inner.write_all(Self::EOF_MARKER)
    }
}

/// This represents the PDF-s cross-reference (xref) table, which is a crucial component that
/// maps each object in the PDF to its location within the file (byte offset from the start).
#[derive(Default)]
pub struct CrossReferenceTable {
    /// Storing solely byte offsets, since we are considering the generation
    /// number to be '00000' and in use flag to be 'n' at all times.
    offsets: Vec<usize>,
}

impl CrossReferenceTable {
    /// Marker representing the start of CRT section (4 characters “xref”).
    const XREF_MARKER: &[u8] = b"xref";

    /// Adds a new object offset to the table.
    fn add_object(&mut self, byte_offset: usize) {
        self.offsets.push(byte_offset);
    }
}

/// This represents one cohesive PDF document that can contain multiple pages of content.
#[derive(Default)]
pub struct Document;

impl Document {
    /// Write the PDF contents into the provided writer.
    pub fn write(&self, writer: &mut impl Write) -> Result<(), io::Error> {
        let mut pdf_writer = PdfWriter::new(writer);
        pdf_writer.write_header()?;
        pdf_writer.write_eof()?;

        Ok(())
    }
}
