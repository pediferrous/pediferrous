//! Implementation of loading and embedding of font programs (files).
#![allow(dead_code)]

use std::{io::Write, path::Path};

use crate::{
    ObjId,
    types::hierarchy::primitives::{
        array::WriteArray as _,
        font::{Font, FontSubtype},
        identifier::Identifier,
    },
};

use pdfgen_macros::const_identifiers;

pub(crate) struct FontMeta {
    pub(crate) name: String,
}

impl FontMeta {
    pub(crate) fn load_from_path(font_file_path: &Path) -> std::io::Result<FontMeta> {
        let font_file = std::fs::read(font_file_path)?;
        Self::load(&font_file)
    }

    pub(crate) fn load(bytes: &[u8]) -> std::io::Result<FontMeta> {
        let face = ttf_parser::Face::parse(bytes, 0)
            .map_err(|_| std::io::Error::other("invalid font file provided"))?;

        let post_script_name = face
            .names()
            .into_iter()
            .find(|name| name.name_id == ttf_parser::name_id::POST_SCRIPT_NAME && name.is_unicode())
            .and_then(|name| name.to_string())
            .ok_or_else(|| std::io::Error::other("font must contain valid name"))?;

        Ok(FontMeta {
            name: post_script_name,
        })
    }
}

pub(crate) struct FontType0 {
    /// The name of the font. If the descendant is a Type 0 CIDFont, this name should be the
    /// concatenation of the CIDFont’s BaseFont name, a hyphen, and the CMap name given in the
    /// Encoding entry (or the CMapName entry in the CMap). If the descendant is a Type 2 CIDFont,
    /// this name should be the same as the CIDFont’s BaseFont name.
    ///
    /// NOTE(nfejzic): In principle, this is an arbitrary name, since there is no font program
    /// associated directly with a Type 0 font dictionary. The conventions described here ensure
    /// maximum compatibility with existing PDF processors.
    post_script_name: Identifier<String>,
    encoding: Identifier<String>, // Identity-H
    descendant_fonts: [ObjId<CidFontType2>; 1],
}

impl FontType0 {
    const_identifiers! {
        ENCODING,
        IDENTITY_H: b"Identity-H",
        DESCENDANT_FONTS,
    }

    pub(crate) fn write(&self, writer: &mut dyn Write) -> Result<usize, std::io::Error> {
        let written = pdfgen_macros::write_chain! {
            Identifier::TYPE.write(writer),
            Identifier::FONT.write(writer),
            writer.write(crate::types::constants::NL_MARKER),

            Font::SUBTYPE.write(writer),
            FontSubtype::Type0.write(writer),
            writer.write(crate::types::constants::NL_MARKER),

            Font::BASE_FONT.write(writer),
            self.post_script_name.write(writer),
            writer.write(crate::types::constants::NL_MARKER),

            Self::ENCODING.write(writer),
            Self::IDENTITY_H.write(writer),
            writer.write(crate::types::constants::NL_MARKER),

            Self::DESCENDANT_FONTS.write(writer),
            self.descendant_fonts.write_array(writer, None),
            writer.write(crate::types::constants::NL_MARKER),
        };

        Ok(written)
    }
}

// /Type /Font
// /Subtype /Type0
// /BaseFont /JetBrainsMono
// /Encoding /Identity-H
// /DescendantFonts [11 0 R]

pub(crate) struct CidFontType2 {}

impl CidFontType2 {
    const_identifiers! {
        FONT_DESCRIPTOR
    }
}
