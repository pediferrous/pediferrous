//! Implementation of PDF Font object.

use crate::types::{self, constants};

use super::{name::Name, obj_id::ObjId, object::Object};

/// Represents a font object in a PDF document.
/// This struct represents a font object in a PDF document, encapsulating the info required to
/// define and reference a font, including its unique ID, subtype, and base font type.
/// Fonts are essential for rendering text in PDFs and specify the appearance and
/// characteristics of text elements.
pub struct Font {
    /// ID of this Font object.
    id: ObjId,

    /// Specifies the subtype of the font, defining its role or characteristics within the PDF.
    subtype: Vec<u8>,

    /// Represents the base font type, identifying the general font family or format.
    base_type: Vec<u8>,
}

impl Font {
    const FONT: Name<&'static [u8]> = Name::from_static(b"Font");
    const SUBTYPE: Name<&'static [u8]> = Name::from_static(b"Subtype");
    const BASE_FONT: Name<&'static [u8]> = Name::from_static(b"BaseFont");

    /// Create a new Font object with the provided id, subtype and base_type.
    pub fn new(id: ObjId, subtype: Vec<u8>, base_type: Vec<u8>) -> Self {
        Font {
            id,
            subtype,
            base_type,
        }
    }

    /// Returns the object reference of this Font object.
    pub fn obj_ref(&self) -> ObjId {
        self.id.clone()
    }
}

impl Object for Font {
    fn write(&self, writer: &mut impl std::io::Write) -> Result<usize, std::io::Error> {
        let subtype_value = Name::new(&self.subtype);
        let base_type_value = Name::new(&self.base_type);

        let bytes_written = types::write_chain! {
            writer.write(b"<< "),

            // /Type /Font
            Name::TYPE.write(writer),
            Self::FONT.write(writer),
            writer.write(constants::NL_MARKER),

            // /Subtype /xyz
            Self::SUBTYPE.write(writer),
            subtype_value.write(writer),
            writer.write(constants::NL_MARKER),

            // /BaseFont /xyz
            Self::BASE_FONT.write(writer),
            base_type_value.write(writer),
            writer.write(constants::NL_MARKER),

            writer.write(b">>"),
        };

        Ok(bytes_written)
    }
}
