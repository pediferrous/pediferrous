//! Definition of PDF object(trait).

use std::io::{self, Write};

use crate::types::constants;

/// The [`Object`] trait serves as a blueprint for all types that need to
/// provide a custom implementation for serializing or outputting their
/// structured data in a consistent manner.
pub(crate) trait Object: std::fmt::Debug {
    /// Writes the object definition part of this object, for example `3 0 obj\n`.
    ///
    /// The newline should be included in the implementation of this function.
    fn write_def(&self, writer: &mut dyn Write) -> Result<usize, io::Error>;

    /// Writes the structured data of the object to the provided writer.
    fn write_content(&self, writer: &mut dyn Write) -> Result<usize, io::Error>;

    /// Writes the `endobj` marker for objects.
    fn write_end(&self, writer: &mut dyn Write) -> Result<usize, io::Error> {
        Ok(pdfgen_macros::write_chain! {
            writer.write(constants::END_OBJ_MARKER),
            writer.write(constants::NL_MARKER),
        })
    }
}
