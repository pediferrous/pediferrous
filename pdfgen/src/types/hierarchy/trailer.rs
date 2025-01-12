//! Implementation of the PDF-s trailer section.

use std::io::Write;

use crate::types::{self, constants};

use super::{
    cross_reference_table::CrossReferenceTable,
    primitives::{array::WriteArray, name::Name, obj_id::ObjId},
};

/// Extension trait for implementations of Trailer sections (currently only CRT).
pub trait WriteTrailer {
    /// Hash crt's data and write it to the given implementor of [`Write`] trait following the
    /// PDF documentations trailer section structure.
    fn write_trailer(
        &self,
        writer: &mut impl Write,
        offset: usize,
        size: usize,
        root: ObjId,
        id: [u8; 16],
    ) -> Result<(), std::io::Error>;
}

impl WriteTrailer for CrossReferenceTable {
    fn write_trailer(
        &self,
        writer: &mut impl Write,
        offset: usize,
        size: usize,
        root: ObjId,
        id: [u8; 16],
    ) -> Result<(), std::io::Error> {
        const SIZE: Name<'static> = Name::from_static(b"Size");
        const ROOT: Name<'static> = Name::from_static(b"Root");
        const ID: Name<'static> = Name::from_static(b"ID");

        /// Marker representing the start of the `trailer` section.
        const TRAILER_MARKER: &[u8] = b"trailer\n";
        /// Marker representing the start of the xref byte offset section.
        const START_XREF_MARKER: &[u8] = b"startxref\n";

        let indent = &constants::SP.repeat(TRAILER_MARKER.len() - 1);
        types::write_chain! {
            // dict start
            writer.write(TRAILER_MARKER),
            writer.write(indent),
            writer.write(b"<< "),
            // Size
            SIZE.write(writer),
            writer.write(size.to_string().as_bytes()),
            writer.write(constants::NL_MARKER),
            // Root
            writer.write(indent),
            ROOT.write(writer),
            root.write_ref(writer),
            writer.write(constants::NL_MARKER),
            // ID
            writer.write(indent),
            ID.write(writer),
            id.write_array(writer, Some(indent.len() + ID.len())),
            writer.write(constants::NL_MARKER),
            // dict end
            writer.write(indent),
            writer.write(b">>"),
            writer.write(constants::NL_MARKER),
            // startxref
            writer.write(START_XREF_MARKER),
            writer.write(offset.to_string().as_bytes()),
            writer.write(constants::NL_MARKER),
        };

        Ok(())
    }
}
