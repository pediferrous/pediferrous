use std::io::{Error, Write};

use crate::types::constants;

use super::obj_id::ObjId;

/// Extension trait for implementations of arrays. This trait should be implemented for array-like
/// data structures that can be used to represent PDF's array primitive type.
pub trait WriteArray {
    /// Encode `self` as PDF array and write it to the given implementor of [`Write`] trait.
    fn write_array(&self, writer: &mut dyn Write, indent: Option<usize>) -> Result<usize, Error>;
}

impl WriteArray for Vec<ObjId> {
    fn write_array(&self, writer: &mut dyn Write, indent: Option<usize>) -> Result<usize, Error> {
        let opening = b"[";

        let mut written = writer.write(opening)?;
        let indent = " ".repeat(indent.unwrap_or(0) + opening.len());

        for (idx, obj_ref) in self.iter().enumerate() {
            if idx > 0 {
                written += writer.write(constants::NL_MARKER)?;
                written += writer.write(indent.as_bytes())?;
            }

            written += obj_ref.write_ref(writer)?;
        }

        written += writer.write(b"]")?;

        Ok(written)
    }
}

impl WriteArray for [u8; 16] {
    fn write_array(&self, writer: &mut dyn Write, indent: Option<usize>) -> Result<usize, Error> {
        let indent = " ".repeat(indent.unwrap_or(0));

        let written = pdfgen_macros::write_chain! {
            writer.write(b"["),
            writer.write(b"<"),
            writer.write(hex::encode(self).as_bytes()),
            writer.write(b">"),
            writer.write(constants::NL_MARKER),
            writer.write(indent.as_bytes()),
            writer.write(b"<"),
            writer.write(hex::encode(self).as_bytes()),
            writer.write(b">"),
            writer.write(constants::NL_MARKER),
            writer.write(indent.as_bytes()),
            writer.write(b"]"),
        };

        Ok(written)
    }
}
