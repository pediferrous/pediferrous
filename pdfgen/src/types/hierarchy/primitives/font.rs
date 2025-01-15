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
    subtype: Name<Vec<u8>>,

    /// Represents the base font type, identifying the general font family or format.
    base_font: Name<Vec<u8>>,
}

impl Font {
    const FONT: Name<&'static [u8]> = Name::from_static(b"Font");
    const SUBTYPE: Name<&'static [u8]> = Name::from_static(b"Subtype");
    const BASE_FONT: Name<&'static [u8]> = Name::from_static(b"BaseFont");

    /// Create a new Font object with the provided id, subtype and base_font.
    pub fn new<S, B>(id: ObjId, subtype: S, base_font: B) -> Self
    where
        S: Into<Vec<u8>>,
        B: Into<Vec<u8>>,
    {
        let subtype = Name::new(subtype.into());
        let base_font = Name::new(base_font.into());

        Font {
            id,
            subtype,
            base_font,
        }
    }

    /// Returns the object reference of this Font object.
    pub fn obj_ref(&self) -> ObjId {
        self.id.clone()
    }
}

impl Object for Font {
    fn write(&self, writer: &mut impl std::io::Write) -> Result<usize, std::io::Error> {
        let bytes_written = types::write_chain! {
            writer.write(b"<< "),

            // /Type /Font
            Name::TYPE.write(writer),
            Self::FONT.write(writer),
            writer.write(constants::NL_MARKER),

            // /Subtype /xyz
            Self::SUBTYPE.write(writer),
            self.subtype.write(writer),
            writer.write(constants::NL_MARKER),

            // /BaseFont /xyz
            Self::BASE_FONT.write(writer),
            self.base_font.write(writer),
            writer.write(constants::NL_MARKER),

            writer.write(b">>"),
        };

        Ok(bytes_written)
    }
}
