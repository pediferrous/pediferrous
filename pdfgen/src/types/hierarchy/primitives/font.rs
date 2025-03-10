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
    /// ID of this [`Font`] object.
    id: ObjId,

    /// Name of this [`Font`], allowing it to be referenced with it.
    name: Name<Vec<u8>>,

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
        NAME,
    }

    /// Create a new [`Font`] object with the provided id, subtype and base_font.
    pub fn new<N, S, B>(name: N, id: ObjId, subtype: S, base_font: B) -> Self
    where
        N: Into<Vec<u8>>,
        S: Into<Vec<u8>>,
        B: Into<Vec<u8>>,
    {
        let name = Name::new(name.into());
        let subtype = Name::new(subtype.into());
        let base_font = Name::new(base_font.into());

        Font {
            id,
            name,
            subtype,
            base_font,
        }
    }

    /// Writes the PDF Font reference, using the FontName and provided size.
    pub fn write_ref(
        &self,
        size: u32,
        writer: &mut dyn std::io::Write,
    ) -> Result<usize, std::io::Error> {
        // /FName size
        Ok(pdfgen_macros::write_chain! {
            self.name.write(writer),
            writer.write(format!{" {}", size}.as_bytes()),
        })
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

            // /Name /xyz
            Self::NAME.write(writer),
            self.name.write(writer),
            writer.write(constants::NL_MARKER),

            writer.write(b">>"),
            writer.write(constants::NL_MARKER),
        };

        Ok(bytes_written)
    }
}
