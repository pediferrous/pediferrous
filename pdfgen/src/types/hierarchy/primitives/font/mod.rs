//! Implementation of PDF Font object.

use std::io::{Error, Write};

use pdfgen_macros::const_identifiers;

use crate::{ObjId, types::constants};

use super::{identifier::Identifier, object::Object};

/// Enumerates the font subtypes defined by ISO 32000 for use in a PDF font dictionary.
///
/// These values correspond to the allowed values of the `/Subtype` key in a PDF
/// font dictionary. The subtype specifies the underlying font technology used
/// to represent the font within the PDF file.
///
/// Only the values defined by ISO 32000 are valid. No other values for the
/// `/Subtype` key are permitted according to the specification.
///
/// Each enum variant maps directly to the name used in the PDF specification
/// and identifies the structure and behavior of the font program embedded
/// or referenced by the PDF.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontSubtype {
    /// A composite font — a font composed of glyphs from a descendant CIDFont (PDF 1.2)
    Type0,

    /// A font that defines glyph shapes using Type 1 font technology
    Type1,

    /// A multiple master font — an extension of the Type 1 font that allows the generation of a
    /// wide variety of typeface styles from a single font
    MMType,
    /// A font that defines glyphs with streams of PDF graphics operators
    Type3,

    // TODO(nfejzic): update the "see" clause
    /// A font based on the TrueType font format (see 9.6.3, "TrueType fonts") and with glyph
    /// descriptions based on TrueType glyph technology.
    TrueType,

    /// A CIDFont whose glyph descriptions are based on CFF font technology (PDF 1.2)
    CidFontType0,

    /// A CIDFont whose glyph descriptions are based on TrueType glyph technology (PDF 1.2)
    CidFontType2,
}

impl FontSubtype {
    pub fn is_embeddable(&self) -> bool {
        matches!(
            self,
            Self::TrueType | Self::Type1 | Self::CidFontType0 | Self::CidFontType2
        )
    }

    pub fn write(&self, writer: &mut dyn Write) -> Result<usize, Error> {
        Identifier::from(*self).write(writer)
    }
}

impl From<FontSubtype> for Identifier<&'static [u8]> {
    fn from(val: FontSubtype) -> Self {
        pdfgen_macros::const_identifiers! {
            TYPE0,
            TYPE1,
            M_M_TYPE,
            TYPE3,
            TRUE_TYPE,
            C_I_D_FONT_TYPE0,
            C_I_D_FONT_TYPE2,
        };

        match val {
            FontSubtype::Type0 => TYPE0,
            FontSubtype::Type1 => TYPE1,
            FontSubtype::MMType => M_M_TYPE,
            FontSubtype::Type3 => TYPE3,
            FontSubtype::TrueType => TRUE_TYPE,
            FontSubtype::CidFontType0 => C_I_D_FONT_TYPE0,
            FontSubtype::CidFontType2 => C_I_D_FONT_TYPE2,
        }
    }
}

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
    subtype: FontSubtype,

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
    pub fn base<B>(id: ObjId<Self>, subtype: FontSubtype, base_font: B) -> Self
    where
        B: Into<Vec<u8>>,
    {
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
    use crate::{
        IdManager,
        types::hierarchy::primitives::font::{FontSubtype, Object},
    };

    use super::Font;

    #[test]
    pub fn font_object() {
        let mut id_manager = IdManager::new();
        let font = Font::base(id_manager.create_id(), FontSubtype::Type1, "Helvetica");

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
