//! Implementation of PDF Font object.

use std::io::{Error, Write};

use pdfgen_macros::const_names;

use crate::{types::constants, ObjId};

use super::{name::Name, object::Object};

/// Represents a font object in a PDF document.
/// This struct represents a font object in a PDF document, encapsulating the info required to
/// define and reference a font, including its unique ID, subtype, and base font type.
/// Fonts are essential for rendering text in PDFs and specify the appearance and
/// characteristics of text elements.
#[derive(Debug)]
pub struct Font {
    /// ID of this Font object.
    id: ObjId,

    /// Specifies the subtype of the font, defining its role or characteristics within the PDF.
    subtype: Name<Vec<u8>>,

    /// Represents the base font type, identifying the general font family or format.
    base_font: Name<Vec<u8>>,
}

impl Font {
    const_names! {
        FONT,
        SUBTYPE,
        BASE_FONT,
    }

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
}

impl Object for Font {
    fn write_def(&self, writer: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        Ok(pdfgen_macros::write_chain! {
            self.id.write_def(writer),
            writer.write(constants::NL_MARKER),
        })
    }

    fn write_content(&self, writer: &mut dyn Write) -> Result<usize, Error> {
        let bytes_written = pdfgen_macros::write_chain! {
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
            writer.write(constants::NL_MARKER),
        };

        Ok(bytes_written)
    }
}
