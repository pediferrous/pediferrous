//! Definition of PDF object(trait).

use std::io::{self, Write};

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
