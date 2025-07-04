//! Implementation of PDF Font object.

use std::io::{Error, Write};

use pdfgen_macros::const_names;

use crate::{types::constants, ObjId};

use super::{
    name::{Name, OwnedName},
    object::Object,
};

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
    name: OwnedName,

    /// Specifies the subtype of the font, defining its role or characteristics within the PDF.
    subtype: OwnedName,

    /// Represents the base font type, identifying the general font family or format.
    base_font: OwnedName,
}

impl Font {
    const_names! {
        FONT,
        SUBTYPE,
        BASE_FONT,
        NAME,
    }

    /// Create a new [`Font`] object with the provided id, subtype and base_font.
    pub fn try_new<N, S, B>(
        name: N,
        id: ObjId,
        subtype: S,
        base_font: B,
    ) -> Result<Self, &'static str>
    where
        N: Into<Vec<u8>>,
        S: Into<Vec<u8>>,
        B: Into<Vec<u8>>,
    {
        let name = Name::try_new(name.into())?;
        let subtype = Name::try_new(subtype.into())?;
        let base_font = Name::try_new(base_font.into())?;

        Ok(Font {
            id,
            name,
            subtype,
            base_font,
        })
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
            writer.write(format!{" {size}"}.as_bytes()),
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

#[cfg(test)]
mod tests {
    use crate::{types::hierarchy::primitives::font::Object, IdManager};

    use super::Font;

    #[test]
    pub fn font_object() {
        let mut id_manager = IdManager::new();
        let font =
            Font::try_new("CustomFnt", id_manager.create_id(), "Type1", "Helvetica").unwrap();

        let mut writer = Vec::default();
        let _ = font.write_def(&mut writer);
        let _ = font.write_content(&mut writer);
        let _ = font.write_end(&mut writer);

        let output = String::from_utf8_lossy(&writer);
        insta::assert_snapshot!(output);
    }
}
