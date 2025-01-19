//! Implementation of the PDF-s cross reference table.

use std::io::Write;

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

    /// Representing the PDF SPLF newline used for crt entries.
    const SP_LF: &str = " \n";

    /// Adds a new object offset to the table.
    pub fn add_object(&mut self, byte_offset: usize) {
        self.offsets.push(byte_offset);
    }

    /// Writes the contents of the `offsets`, representing them in the format required by the PDF
    /// syntax, `10 byte offset generation(00000), n`.
    pub fn write(&self, writer: &mut impl Write) -> Result<(), std::io::Error> {
        pdfgen_macros::write_chain! {
            writer.write(Self::XREF_MARKER),
            writer.write(format!("0 {}\n", self.offsets.len()).as_bytes()),

            for offset in self.offsets.iter() {
                writer.write(format!("{offset:010} 00000 n{}", Self::SP_LF).as_bytes()),
            },
        };

        Ok(())
    }

    /// Returns length(size) of the `offsets` collection.
    pub fn len(&self) -> usize {
        self.offsets.len()
    }

    /// Returns if the `offsets` collection is empty or not.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Computes the 16b MD5 hash of the `offsets` collection.
    pub fn offsets_hash(&self) -> Result<[u8; 16], std::io::Error> {
        let bytes: Vec<u8> = self
            .offsets
            .iter()
            .flat_map(|&offset| offset.to_ne_bytes())
            .collect();

        Ok(*md5::compute(&bytes))
    }
}
