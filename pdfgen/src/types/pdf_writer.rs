//! Implementation of the [`PdfWriter`] wrapper.

use crate::{IdManager, ObjId};

use super::{
    constants,
    hierarchy::{
        catalog::Catalog, cross_reference_table::CrossReferenceTable, primitives::object::Object,
        trailer::WriteTrailer,
    },
    page::Page,
};
use std::io::{self, Write};

/// A wrapper around any type that implements [`Write`], adding pdf specific functionality to keep a
/// clear and consistent CrossReferenceTable state
pub struct PdfWriter<W: Write> {
    /// Inner member, representing a type that implements [`Write`].
    inner: W,
    /// Current byte offset from the top of document, representing the current position of the `cursor`.
    current_offset: usize,
    /// CrossReferenceTable member, representing the current state of the cross_reference_table
    /// for the document
    cross_reference_table: CrossReferenceTable,
}

impl<W: Write> PdfWriter<W> {
    /// The PDF file begins with the 5 characters “%PDF–X.X” and byte offsets shall be calculated
    /// from the PERCENT SIGN.
    const PDF_HEADER: &[u8] = b"%PDF-2.0";
    /// The last line of the file shall contain only the end-of-file marker, %%EOF
    const EOF_MARKER: &[u8] = b"%%EOF";

    /// Creates a new [`PdfWriter`] instance.
    pub fn new(inner: W) -> Self {
        PdfWriter {
            inner,
            // NOTE: The current byte is included in offset.
            current_offset: 1,
            cross_reference_table: CrossReferenceTable::default(),
        }
    }

    /// Write the PDF documents header marker updating the `cursor`s byte offset with the number of
    /// bytes written.
    pub fn write_header(&mut self) -> Result<(), io::Error> {
        // Delegate the actual writing to the inner writer incrementing the current_offset to
        // reflect current `cursor` position.
        self.current_offset += self.inner.write(Self::PDF_HEADER)?;
        self.current_offset += self.inner.write(constants::NL_MARKER)?;

        Ok(())
    }

    /// Writes the object start marker(`X X obj`), following with the structured data of the object
    /// itself, finalizing with object end marker(`endobj`), ensuring correct CrossReferenceTable
    /// and cursor update.
    pub(crate) fn write_object(&mut self, obj: &dyn Object) -> Result<(), io::Error> {
        // Save the objects byte offset in the CrossReferenceTable.
        self.cross_reference_table.add_object(self.current_offset);

        // X Y obj\n
        self.current_offset += obj.write_def(&mut self.inner)?;

        // Delegate the actual writing to the inner writer.
        self.current_offset += obj.write_content(&mut self.inner)?;

        // endobj\n
        self.current_offset += obj.write_end(&mut self.inner)?;

        // spacing for readability
        self.current_offset += self.inner.write(constants::NL_MARKER)?;

        Ok(())
    }

    /// Writes the cross reference table contents.
    pub fn write_crt(&mut self) -> Result<(), io::Error> {
        self.cross_reference_table.write(&mut self.inner)?;

        Ok(())
    }

    /// Writes the trailer for the PdfWriter's CRT.
    pub fn write_trailer(&mut self, root: ObjId<Catalog>) -> Result<(), io::Error> {
        self.cross_reference_table.write_trailer(
            &mut self.inner,
            self.current_offset,
            self.cross_reference_table.len(),
            root,
            self.cross_reference_table.offsets_hash()?,
        )?;

        Ok(())
    }

    /// Write the PDF documents EOF marker.
    pub fn write_eof(&mut self) -> Result<(), io::Error> {
        // Delegate the actual writing to the inner writer.
        self.inner.write_all(Self::EOF_MARKER)
    }

    /// Writes the page contents into the PDF document.
    pub(crate) fn write_page(
        &mut self,
        page: &Page,
        id_manager: &mut IdManager,
    ) -> Result<(), io::Error> {
        self.cross_reference_table.add_object(self.current_offset);

        let (bytes_written, offsets) = page.write(&mut self.inner, id_manager)?;

        for offset in offsets {
            self.cross_reference_table
                .add_object(self.current_offset + offset);
        }

        self.current_offset += bytes_written;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        IdManager, ObjId,
        types::{constants, pdf_writer::PdfWriter},
    };

    use super::Object;

    #[derive(Debug)]
    struct Dummy(ObjId);

    impl Object for Dummy {
        fn write_def(&self, writer: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
            Ok(pdfgen_macros::write_chain! {
                self.0.write_def(writer),
                writer.write(constants::NL_MARKER),
            })
        }

        fn write_content(&self, writer: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
            writer.write(b"FirstLine\nSecondLine\n")
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
        let mut id_manager = IdManager::new();

        let dummy = Dummy(id_manager.create_id());
        pdf_writer.write_object(&dummy).unwrap();

        let output = String::from_utf8(writer).unwrap();

        insta::assert_snapshot!(
            output,
            @r"
        1 0 obj
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
        let mut id_manager = IdManager::new();

        pdf_writer.write_header().unwrap();
        let dummy = Dummy(id_manager.create_id());
        pdf_writer.write_object(&dummy).unwrap();
        let dummy = Dummy(id_manager.create_id());
        pdf_writer.write_object(&dummy).unwrap();
        let dummy = Dummy(id_manager.create_id());
        pdf_writer.write_object(&dummy).unwrap();
        let dummy = Dummy(id_manager.create_id());
        pdf_writer.write_object(&dummy).unwrap();

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
        0000000010 00000 n 
        0000000047 00000 n 
        0000000084 00000 n 
        0000000121 00000 n 
        %%EOF
        "
        );
    }

    #[test]
    fn write_trailer() {
        let mut writer = Vec::new();
        let mut pdf_writer = PdfWriter::new(&mut writer);
        let mut id_manager = IdManager::new();

        pdf_writer.write_header().unwrap();
        let dummy = Dummy(id_manager.create_id());
        pdf_writer.write_object(&dummy).unwrap();
        let dummy = Dummy(id_manager.create_id());
        pdf_writer.write_object(&dummy).unwrap();
        let dummy = Dummy(id_manager.create_id());
        pdf_writer.write_object(&dummy).unwrap();
        let dummy = Dummy(id_manager.create_id());
        pdf_writer.write_object(&dummy).unwrap();
        pdf_writer.write_crt().unwrap();
        pdf_writer.write_trailer(id_manager.create_id()).unwrap();
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
        0000000010 00000 n 
        0000000047 00000 n 
        0000000084 00000 n 
        0000000121 00000 n 
        trailer
               << /Size 4
               /Root 5 0 R
               /ID [<ffb2e086bea707d8d867d4a23074276b>
                  <ffb2e086bea707d8d867d4a23074276b>
                  ]
               >>
        startxref
        158
        %%EOF
        "
        );
    }
}
