use std::io::Write;

/// Bytes containing ASCII characters.
pub struct AsciiString {}

/// A series of bytes that shall represent characters or other binary data. If such a type
/// represents characters, the encoding shall be determined by the context.
pub struct ByteString {}

/// Bytes containing a string that shall be encoded using PDFDocEncoding.
///
/// PDFDocEncoding can encode all of the ISO Latin 1 character set. PDFDocEncoding does not support
/// all Unicode characters whereas UTF-16BE and UTF-8 do.
pub struct PdfDocEncString {}

/// Any string that is not a text string. Beginning with PDF 1.7, this type is further qualified as
/// the types: ASCII string and byte string.
pub enum AnyString {
    Ascii(AsciiString),
    Byte(ByteString),
}

/// Bytes that represent characters that shall be encoded using either PDFDocEncoding, UTF-16BE or
/// UTF-8 (as defined in 7.9.2.2, "Text string type".)
///
/// The text string type shall be used for character strings that contain information intended to
/// be human-readable, such as text annotations, document outline item names, article names, and
/// so forth.
pub struct TextString {
    /// UTF-8 encoded string, which is one of allowed encodings for PDF's TextString.
    inner: String,
}

impl TextString {
    /// Returns the leading bytes of UTF-8 PDF encoding.
    #[inline(always)]
    pub fn leading_bytes(&self) -> [u8; 3] {
        // NOTE: For text strings encoded in UTF-16BE, the first two bytes shall be 254 followed by
        //       255.
        [239, 187, 191]
    }

    #[inline(always)]
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        writer.write_all(&self.leading_bytes())?;
        writer.write_all(self.inner.as_bytes())
    }
}
