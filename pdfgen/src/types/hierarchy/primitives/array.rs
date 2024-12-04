use std::io::{Error, Write};

use crate::types::constants;

use super::obj_ref::ObjRef;

/// Extension trait for implementations of arrays. This trait should be implemented for array-like
/// data structures that can be used to represent PDF's array primitive type.
pub trait WriteArray {
    /// Encode `self` as PDF array and write it to the given implementor of [`Write`] trait.
    fn write_array(&self, writer: &mut impl Write, indent: Option<usize>) -> Result<usize, Error>;
}

impl WriteArray for Vec<ObjRef> {
    fn write_array(&self, writer: &mut impl Write, indent: Option<usize>) -> Result<usize, Error> {
        let mut written = writer.write(b"[")?;
        let indent = " ".repeat(indent.unwrap_or(0));

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
