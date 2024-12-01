#![forbid(unsafe_code)]

//! `pdfgen` is a low-level library that offers fine-grained control over PDF syntax and
//! PDF file generation.

use std::io::{self, Write};

use types::hierarchy::primitives::obj_ref::ObjRef;

pub mod types;

/// The [`Object`] trait serves as a blueprint for all types that need to
/// provide a custom implementation for serializing or outputting their
/// structured data in a consistent manner.
pub trait Object {
    /// Writes the structured data of the object to the provided writer.
    ///
    /// # Arguments
    ///
    /// * `writer` - A mutable reference to a type that implements the [`Write`](std::io::Write) trait.
    ///   This is where the object's structured data will be written to.
    ///
    /// # Returns
    ///
    /// A `Result` containing the number of bytes written if successful, or an [`io::Error`](std::io::Error)
    /// if the operation fails.
    fn write(&self, writer: &mut impl Write) -> Result<usize, io::Error>;
}

/// A wrapper around any type that implements [`Write`], adding pdf specific functionality to keep a
/// clear and consistent CrossReferenceTable state
pub struct PdfWriter<W: Write> {
    /// Inner member, representing a type that implements [`Write`].
    inner: W,
    /// Current byte offset from the top of document, representing the current position of the `cursor`.
    current_offset: usize,
    /// Comment
    cross_reference_table: CrossReferenceTable,
}

impl<W: Write> PdfWriter<W> {
    /// The PDF file begins with the 5 characters “%PDF–X.X” and byte offsets shall be calculated
    /// from the PERCENT SIGN.
    const PDF_HEADER: &[u8] = b"%PDF-2.0";
    /// The last line of the file shall contain only the end-of-file marker, %%EOF
    const EOF_MARKER: &[u8] = b"%%EOF";
    /// New line constant
    const NL_MARKER: &[u8] = b"\n";
    /// Marker indicating end of an object section
    const END_OBJ_MARKER: &[u8] = b"endobj";

    /// Creates a new [`PdfWriter`] instance.
    pub fn new(inner: W) -> Self {
        PdfWriter {
            inner,
            current_offset: 0,
            cross_reference_table: CrossReferenceTable::default(),
        }
    }

    /// Write the PDF documents header marker updating the `cursor`s byte offset with the number of
    /// bytes written
    pub fn write_header(&mut self) -> Result<(), io::Error> {
        // Delegate the actual writing to the inner writer incrementing the current_offset to
        // reflect current `cursor` position.
        self.current_offset += self.inner.write(Self::PDF_HEADER)?;
        self.current_offset += self.inner.write(Self::NL_MARKER)?;

        Ok(())
    }

    /// Writes the object start marker(`X X obj`), following with the structured data of the object
    /// itself, finalizing with object end marker(`endobj`), ensuring correct CrossReferenceTable
    /// and cursor update.
    pub fn write_object(&mut self, obj: &impl Object, obj_ref: ObjRef) -> Result<(), io::Error> {
        // Save the objects byte offset in the CrossReferenceTable.
        self.cross_reference_table.add_object(self.current_offset);

        // X Y obj\n
        self.current_offset += obj_ref.write_def(&mut self.inner)?;
        self.current_offset += self.inner.write(Self::NL_MARKER)?;

        // Delegate the actual writing to the inner writer.
        self.current_offset += obj.write(&mut self.inner)?;
        self.current_offset += self.inner.write(Self::NL_MARKER)?;

        // endobj\n
        self.current_offset += self.inner.write(Self::END_OBJ_MARKER)?;
        self.current_offset += self.inner.write(Self::NL_MARKER)?;

        Ok(())
    }

    /// Comment
    pub fn write_crt(&mut self) -> Result<(), io::Error> {
        self.current_offset += self.cross_reference_table.write(&mut self.inner)?;

        Ok(())
    }

    /// Write the PDF documents EOF marker.
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
    /// number to be `00000` and in use flag to be `n` at all times.
    offsets: Vec<usize>,
}

impl CrossReferenceTable {
    /// Marker representing the start of CRT section (4 characters “xref”).
    const XREF_MARKER: &[u8] = b"xref\n";

    /// Comment
    const SP_LF: &str = " \n";

    /// Adds a new object offset to the table.
    fn add_object(&mut self, byte_offset: usize) {
        self.offsets.push(byte_offset);
    }

    /// Comment
    fn write(&self, writer: &mut impl Write) -> Result<usize, std::io::Error> {
        let written = types::write_chain! {
            writer.write(Self::XREF_MARKER),
            writer.write(format!("0 {}\n", self.offsets.len()).as_bytes()),
            self.offsets.iter()
                .map(|offset| writer.write(format!("{offset:010} 00000 n{}", Self::SP_LF).as_bytes()))
                .sum::<Result<usize, _>>(),
        };

        Ok(written)
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
        // Write object(s) here!
        pdf_writer.write_crt()?;
        pdf_writer.write_eof()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{types::hierarchy::primitives::obj_ref::ObjRef, PdfWriter};

    use super::Object;

    struct Dummy;
    impl Object for Dummy {
        fn write(&self, writer: &mut impl std::io::Write) -> Result<usize, std::io::Error> {
            writer.write(b"FirstLine\nSecondLine")
        }
    }

    #[test]
    fn write_header() {
        let mut writer = Vec::new();
        let mut pdf_writer = PdfWriter::new(&mut writer);

        pdf_writer.write_header().unwrap();

        let output = String::from_utf8(writer).unwrap();

        insta::assert_snapshot!(
            output,
            @"%PDF-2.0"
        );
    }

    #[test]
    fn write_eof() {
        let mut writer = Vec::new();
        let mut pdf_writer = PdfWriter::new(&mut writer);

        pdf_writer.write_eof().unwrap();

        let output = String::from_utf8(writer).unwrap();

        insta::assert_snapshot!(
            output,
            @"%%EOF"
        );
    }

    #[test]
    fn write_object() {
        let mut writer = Vec::new();
        let mut pdf_writer = PdfWriter::new(&mut writer);

        pdf_writer.write_object(&Dummy, ObjRef::from(17)).unwrap();

        let output = String::from_utf8(writer).unwrap();

        insta::assert_snapshot!(
            output,
            @r"
        17 0 obj
        FirstLine
        SecondLine
        endobj
        "
        );
    }

    #[test]
    fn write_crt() {
        let mut writer = Vec::new();
        let mut pdf_writer = PdfWriter::new(&mut writer);

        pdf_writer.write_header().unwrap();
        pdf_writer.write_object(&Dummy, ObjRef::from(1)).unwrap();
        pdf_writer.write_object(&Dummy, ObjRef::from(2)).unwrap();
        pdf_writer.write_object(&Dummy, ObjRef::from(3)).unwrap();
        pdf_writer.write_object(&Dummy, ObjRef::from(4)).unwrap();
        pdf_writer.write_crt().unwrap();
        pdf_writer.write_eof().unwrap();

        let output = String::from_utf8(writer).unwrap();

        insta::assert_snapshot!(
            output,
            @r"
        %PDF-2.0
        1 0 obj
        FirstLine
        SecondLine
        endobj
        2 0 obj
        FirstLine
        SecondLine
        endobj
        3 0 obj
        FirstLine
        SecondLine
        endobj
        4 0 obj
        FirstLine
        SecondLine
        endobj
        xref
        0 4
        0000000009 00000 n 
        0000000045 00000 n 
        0000000081 00000 n 
        0000000117 00000 n 
        %%EOF
        "
        );
    }
}
