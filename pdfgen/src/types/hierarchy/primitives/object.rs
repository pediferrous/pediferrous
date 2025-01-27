//! Definition of PDF object(trait).

use std::io::{self, Write};

use crate::types::constants;

use super::obj_id::{IdManager, ObjId};

/// The [`Object`] trait serves as a blueprint for all types that need to
/// provide a custom implementation for serializing or outputting their
/// structured data in a consistent manner.
pub(crate) trait Object: std::fmt::Debug {
    fn write(&mut self, writer: &mut dyn Write) -> Result<usize, io::Error> {
        Ok(pdfgen_macros::write_chain! {
            self.write_def(writer),
            self.write_content(writer, &mut IdManager::default()),
            self.write_end(writer),
        })
    }

    /// Writes the object definition part of this object, for example `3 0 obj\n`.
    ///
    /// The newline should be included in the implementation of this function.
    fn write_def(&mut self, writer: &mut dyn Write) -> Result<usize, io::Error>;

    /// Writes the structured data of the object to the provided writer.
    fn write_content(
        &mut self,
        writer: &mut dyn Write,
        // TODO: remove this parameter of the function.
        id_manager: &mut IdManager,
    ) -> Result<usize, io::Error>;

    /// Writes the `endobj` marker for objects.
    fn write_end(&mut self, writer: &mut dyn Write) -> Result<usize, io::Error> {
        Ok(pdfgen_macros::write_chain! {
            writer.write(constants::END_OBJ_MARKER),
            writer.write(constants::NL_MARKER),
        })
    }

    /// Returns the [`ObjId`] associated with this object.
    fn obj_ref(&self) -> &ObjId;
}
