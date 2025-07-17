//! Implementation of PDF Font object.

use std::io::{Error, Write};

use pdfgen_macros::const_identifiers;

use crate::{ObjId, types::constants};

use super::{identifier::Identifier, object::Object};

/// Represents a font object in a PDF document.
/// This struct represents a font object in a PDF document, encapsulating the info required to
/// define and reference a font, including its unique ID, subtype, and base font type.
/// Fonts are essential for rendering text in PDFs and specify the appearance and
/// characteristics of text elements.
#[derive(Debug)]
pub struct Font {
    /// ID of this [`Font`] object.
    pub(crate) id: ObjId<Self>,

    /// Specifies the subtype of the font, defining its role or characteristics within the PDF.
    subtype: Identifier<Vec<u8>>,

    /// Represents the base font type, identifying the general font family or format.
    base_font: Identifier<Vec<u8>>,
}

impl Font {
    const_identifiers! {
        FONT,
        SUBTYPE,
        BASE_FONT,
    }

    /// Create a new [`Font`] object with the provided id, subtype and base_font.
    pub fn new<S, B>(id: ObjId<Self>, subtype: S, base_font: B) -> Self
    where
        S: Into<Vec<u8>>,
        B: Into<Vec<u8>>,
    {
        let subtype = Identifier::new(subtype.into());
        let base_font = Identifier::new(base_font.into());

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
            Identifier::TYPE.write(writer),
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

#[cfg(test)]
mod tests {
    use crate::{IdManager, types::hierarchy::primitives::font::Object};

    use super::Font;

    #[test]
    pub fn font_object() {
        let mut id_manager = IdManager::new();
        let font = Font::new(id_manager.create_id(), "Type1", "Helvetica");

        let mut writer = Vec::default();
        let _ = font.write_def(&mut writer);
        let _ = font.write_content(&mut writer);
        let _ = font.write_end(&mut writer);

        let output = String::from_utf8_lossy(&writer);
        insta::assert_snapshot!(output, @r"
        1 0 obj
        << /Type /Font 
        /Subtype /Type1 
        /BaseFont /Helvetica 
        >>
        endobj
        ");
    }
}
