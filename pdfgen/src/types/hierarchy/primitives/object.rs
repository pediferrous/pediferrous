//! Definition of PDF object(trait).

use std::io::{self, Write};

use super::obj_id::ObjId;

/// The [`Object`] trait serves as a blueprint for all types that need to
/// provide a custom implementation for serializing or outputting their
/// structured data in a consistent manner.
pub trait Object {
    /// Writes the structured data of the object to the provided writer.
    fn write(&self, writer: &mut impl Write) -> Result<usize, io::Error>;

    /// Returns the [`ObjId`] associated with this object.
    fn obj_ref(&self) -> &ObjId;
}
