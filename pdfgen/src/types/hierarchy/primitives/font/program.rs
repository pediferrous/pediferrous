//! Implementation of loading and embedding of font programs (files).
#![allow(dead_code)]

use std::{io::Write, path::Path};

use crate::{
    ObjId,
    types::{
        constants,
        hierarchy::{
            content::stream::Stream,
            primitives::{
                array::WriteArray as _,
                font::{Font, FontSubtype},
                identifier::Identifier,
                rectangle::Rectangle,
            },
        },
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

pub(crate) trait FontOptions {
    fn write(&self, writer: &mut dyn Write) -> Result<usize, std::io::Error>;

    fn subtype(&self) -> FontSubtype;
}

pub(crate) struct FontType0 {
    encoding: Identifier<String>,
    descendant_fonts: [ObjId<CidFontType2>; 1],
}

impl FontType0 {
    const_identifiers! {
        ENCODING,
        IDENTITY_H: b"Identity-H",
        DESCENDANT_FONTS,
    }
}

impl FontOptions for FontType0 {
    fn write(&self, writer: &mut dyn Write) -> Result<usize, std::io::Error> {
        let written = pdfgen_macros::write_chain! {
            Self::ENCODING.write(writer),
            Self::IDENTITY_H.write(writer),
            writer.write(constants::NL_MARKER),

            Self::DESCENDANT_FONTS.write(writer),
            self.descendant_fonts.write_array(writer, None),
        };

        Ok(written)
    }

    fn subtype(&self) -> FontSubtype {
        FontSubtype::Type0
    }
}

pub(crate) struct EmbeddedFont<T> {
    /// The name of the font. If the descendant is a Type 0 CIDFont, this name should be the
    /// concatenation of the CIDFont’s BaseFont name, a hyphen, and the CMap name given in the
    /// Encoding entry (or the CMapName entry in the CMap). If the descendant is a Type 2 CIDFont,
    /// this name should be the same as the CIDFont’s BaseFont name.
    ///
    /// NOTE(nfejzic): In principle, this is an arbitrary name, since there is no font program
    /// associated directly with a Type 0 font dictionary. The conventions described here ensure
    /// maximum compatibility with existing PDF processors.
    base_font: Identifier<String>,
    font_options: T,
}

impl<T> EmbeddedFont<T>
where
    T: FontOptions,
{
    pub(crate) fn write(&self, writer: &mut dyn Write) -> Result<usize, std::io::Error> {
        let written = pdfgen_macros::write_chain! {
            Identifier::TYPE.write(writer),
            Identifier::FONT.write(writer),
            writer.write(constants::NL_MARKER),

            Font::SUBTYPE.write(writer),
            self.font_options.subtype().write(writer),
            writer.write(constants::NL_MARKER),

            Font::BASE_FONT.write(writer),
            self.base_font.write(writer),
            writer.write(constants::NL_MARKER),

            self.font_options.write(writer),
            writer.write(constants::NL_MARKER),
        };

        Ok(written)
    }
}

/// The value of the Flags entry in a font descriptor shall be an unsigned 32-bit integer
/// containing flags specifying various characteristics of the font. Bit positions within the flag
/// word are numbered from 1 (low-order) to 32 (high-order).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FontFlags {
    /// All glyphs have the same width (as opposed to proportional or variable-pitch fonts, which
    /// have different widths).
    ///
    /// Bit position: 1
    fixed_pitch: bool,

    /// Glyphs have serifs, which are short strokes drawn at an angle on the top and bottom of
    /// glyph stems. (Sans serif fonts do not have serifs.)
    serif: bool,

    /// Font contains glyphs outside the Standard Latin character set. This flag and the
    /// Nonsymbolic flag shall not both be set or both be clear.
    symbolic: bool,

    /// Glyphs resemble cursive handwriting.
    script: bool,

    /// Font uses the Standard Latin character set or a subset of it. This flag and the Symbolic
    /// flag shall not both be set or both be clear.
    non_symbolic: bool,

    /// Glyphs have dominant vertical strokes that are slanted.
    italic: bool,

    /// Font contains no lowercase letters; typically used for display purposes, such as for titles
    /// or headlines.
    all_cap: bool,

    /// Font contains both uppercase and lowercase letters. The uppercase letters are similar to
    /// those in the regular version of the same typeface family. The glyphs for the lowercase
    /// letters have the same shapes as the corresponding uppercase letters, but they are sized and
    /// their proportions adjusted so that they have the same size and stroke weight as lowercase
    /// glyphs in the same typeface family.
    small_cap: bool,

    /// Determines whether bold glyphs shall be painted with extra pixels even at very small text
    /// sizes by a PDF processor. If the flag is set, features of bold glyphs may be thickened at
    /// small text sizes.
    force_bold: bool,
}

impl From<FontFlags> for u32 {
    fn from(ff: FontFlags) -> Self {
        debug_assert!(
            ff.symbolic != ff.non_symbolic,
            "Symbolic and NonSymbolic flag shall not both be set or both be clear"
        );

        let mut res = 0u32;

        fn to_u32(val: bool, at: usize) -> u32 {
            let offs = at.saturating_sub(1);
            let bit = if val { 1 } else { 0 };
            bit << offs
        }

        res |= to_u32(ff.fixed_pitch, 1);
        res |= to_u32(ff.serif, 2);
        res |= to_u32(ff.symbolic, 3);
        res |= to_u32(ff.script, 4);
        res |= to_u32(ff.non_symbolic, 6);
        res |= to_u32(ff.italic, 7);
        res |= to_u32(ff.all_cap, 17);
        res |= to_u32(ff.small_cap, 18);
        res |= to_u32(ff.force_bold, 19);

        res
    }
}

/// A font descriptor specifies metrics and other attributes of a simple font or a CIDFont as a
/// whole, as distinct from the metrics of individual glyphs. These font metrics provide
/// information that enables a PDF processor to synthesise a substitute font or select a similar
/// font when the font program is unavailable. The font descriptor may also be used to embed the
/// font program in the PDF file.
// WARN(nfejzic): Font descriptors shall not be used with Type 0 fonts. Beginning with PDF 1.5,
// font descriptors may be used with Type 3 fonts.
pub(crate) struct FontDescriptor {
    // The PostScript name of the font.
    font_name: Identifier<String>,

    // A collection of flags defining various characteristics of the font.
    flags: FontFlags,

    // A [`Rectangle`] expressed in the glyph coordinate system, that shall specify the font
    // bounding box. This should be the smallest rectangle enclosing the shape that would result if
    // all of the glyphs of the font were placed with their origins coincident and then filled.
    font_bounding_box: Rectangle,

    /// The angle, expressed in degrees counterclockwise from the vertical, of the dominant
    /// vertical strokes of the font. For example, 9-o'clock position is 90 degrees, and the
    /// 3-o'clock position is -90 degrees.
    ///
    /// The value shall be negative for fonts that slope to the right, as almost all italic fonts
    /// do.
    italic_angle: f64,

    /// The maximum height above the
    /// baseline reached by glyphs in this font. The height of glyphs for accented
    /// characters shall be excluded.
    ascent: f64,

    /// The maximum depth below the baseline reached by glyphs in this font. The value shall be a
    /// negative number.
    descent: f64,

    /// The vertical coordinate of the top of flat capital letters, measured from the baseline.
    cap_height: f64,

    /// The thickness measured horizontally, of the dominant vertical stems of glyphs in the font.
    /// Values shall be positive. A value of 0 indicates an unknown stem thickness.
    stem_v: f64,

    /// A (reference to a) stream containing a TrueType font program.
    font_file2: ObjId<Stream>,
}

impl FontDescriptor {
    const_identifiers! {
        FONT_DESCRIPTOR,
        FONT_NAME,
        FLAGS,
        FONT_BBOX: b"FontBBox",
        ITALIC_ANGLE,
        ASCENT,
        DESCENT,
        CAP_HEIGHT,
        STEM_V,
    }

    pub(crate) fn write(&self, writer: &mut dyn Write) -> Result<usize, std::io::Error> {
        let written = pdfgen_macros::write_chain! {
            Identifier::TYPE.write(writer),
            Self::FONT_DESCRIPTOR.write(writer),
            writer.write(constants::NL_MARKER),

            Self::FONT_NAME.write(writer),
            self.font_name.write(writer),
            writer.write(constants::NL_MARKER),

            Self::FLAGS.write(writer),
            crate::write_fmt!(&mut *writer, u32::from(self.flags)),
            writer.write(constants::NL_MARKER),

            Self::FONT_BBOX.write(writer),
            self.font_bounding_box.write(writer),
            writer.write(constants::NL_MARKER),

            Self::ITALIC_ANGLE.write(writer),
            crate::write_fmt!(&mut *writer, self.italic_angle),
            writer.write(constants::NL_MARKER),

            Self::ASCENT.write(writer),
            crate::write_fmt!(&mut *writer, self.ascent),
            writer.write(constants::NL_MARKER),

            Self::DESCENT.write(writer),
            crate::write_fmt!(&mut *writer, self.descent),
            writer.write(constants::NL_MARKER),

            Self::CAP_HEIGHT.write(writer),
            crate::write_fmt!(&mut *writer, self.cap_height),
            writer.write(constants::NL_MARKER),

            Self::STEM_V.write(writer),
            crate::write_fmt!(&mut *writer, self.stem_v),
            writer.write(constants::NL_MARKER),
        };

        Ok(written)
    }
}

/// The CIDSystemInfo entry in a [`CidFontType2`] is a dictionary that shall specify the
/// [`CidFontType2`]’s character collection. The [`CidFontType2`] need not contain glyph
/// descriptions for all the CIDs in a collection; it may contain a subset. The [`CidSystemInfo`]
/// entry in a CMap file shall be either a single dictionary or an array of dictionaries, depending
/// on whether it associates codes with a single character collection or with multiple character
/// collections
pub(crate) struct CidSystemInfo {
    /// A string identifying the issuer of the character collection. The string shall begin with
    /// the 4 or 5 characters of a registered developer prefix followed by a LOW LINE (5Fh)
    /// followed by any other identifying characters chosen by the issuer. See Annex E, "Extending
    /// PDF", for how to obtain a unique developer prefix.
    registry: String,

    /// A string that uniquely names the character collection within the specified registry.
    ordering: String,

    /// The supplement number of the character collection. An original character collection has a
    /// supplement number of 0. Whenever additional CIDs are assigned in a character collection,
    /// the supplement number shall be increased. Supplements shall not alter the ordering of
    /// existing CIDs in the character collection. This value shall not be used in determining
    /// compatibility between character collections.
    supplementer: usize,
}

impl CidSystemInfo {
    const_identifiers! {
        REGISTRY,
        ORDERING,
        SUPPLEMENTER,
    }

    pub(crate) fn write(&self, writer: &mut dyn Write) -> Result<usize, std::io::Error> {
        let written = pdfgen_macros::write_chain! {
            Self::REGISTRY.write(writer),
            crate::write_fmt!(&mut *writer, self.registry),
            writer.write(constants::NL_MARKER),

            Self::ORDERING.write(writer),
            crate::write_fmt!(&mut *writer, self.ordering),
            writer.write(constants::NL_MARKER),

            Self::SUPPLEMENTER.write(writer),
            crate::write_fmt!(&mut *writer, self.supplementer),
        };

        Ok(written)
    }
}

pub(crate) struct CidFontType2 {
    cid_system_info: CidSystemInfo,
    font_descriptor: ObjId<FontDescriptor>,
    widths: Vec<f64>,
}

impl CidFontType2 {
    const_identifiers! {
        CID_SYSTEM_INFO: b"CIDSystemInfo",
    }
}

impl FontOptions for CidFontType2 {
    fn write(&self, writer: &mut dyn Write) -> Result<usize, std::io::Error> {
        let written = pdfgen_macros::write_chain! {
            Self::CID_SYSTEM_INFO.write(writer),
            self.cid_system_info.write(writer),
            writer.write(constants::NL_MARKER),

            FontDescriptor::FONT_DESCRIPTOR.write(writer),
            self.font_descriptor.write_ref(writer),
        };

        Ok(written)
    }

    fn subtype(&self) -> FontSubtype {
        FontSubtype::CidFontType2
    }
}
