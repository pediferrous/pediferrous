use std::io::{Error, Write};

use crate::types::WriteDictValue;

/// [`Name`] object is an atomic symbol uniquely defined by a sequence of any characters (8-bit
/// values) except null (character code 0) that follow these rules:
///
/// * A NUMBER SIGN (23h) (#) in a name shall be written by using its 2-digit hexadecimal code
///   (23), preceded by the NUMBER SIGN.
/// * Any character in a name that is a regular character (other than NUMBER SIGN) shall be written
///   as itself or by using its 2-digit hexadecimal code, preceded by the NUMBER SIGN.
/// * Any character that is not a regular character shall be written using its 2-digit hexadecimal
///   code, preceded by the NUMBER SIGN only.
///
/// When writing a name in a PDF file, a SOLIDUS (2Fh) (/) shall be used to introduce a name.
/// No token delimiter (such as white-space) occurs between the SOLIDUS and the encoded name.
/// Whitespace used as part of a name shall always be coded using the 2-digit hexadecimal notation.
pub(crate) struct Name(&'static [u8]);

impl Name {
    pub(crate) const TYPE: Name = Name::new(b"Type");

    /// Create a new [`Name`] from the given byte slice. The byte slice must contain at least two
    /// bytes and must not contain '/'.
    ///
    /// # Example
    ///
    /// ```ignore
    /// const PDF_KEY: Name = Name::new(b"PdfKey");
    ///
    /// let mut out_buf = Vec::new();
    /// PDF_KEY.write(&mut out_buf).unwrap();
    /// assert_eq!(&out_buf, b"/PdfKey");
    /// ```
    pub(crate) const fn new<const N: usize>(inner: &'static [u8; N]) -> Self {
        if N == 0 {
            panic!("Dictionary Key must start with '/' followed by at least one ASCII character.");
        }

        let mut i = 0;

        while i < N {
            if inner[i] == b'/' {
                panic!("Dictionary Key is not allowed to contain '/'.");
            }

            i += 1;
        }

        Self(inner)
    }

    /// Encode and write this `Name` into the provided implementor of [`Write`].
    pub(crate) fn write(&self, writer: &mut impl Write) -> Result<usize, Error> {
        let mut written = writer.write(b"/")?;
        written += writer.write(self.0)?;
        written += writer.write(b" ")?;
        Ok(written)
    }

    /// The number of bytes that this `Name` occupies when written into the PDF document. This does
    /// not include the whitespace written after the `Name`.
    ///
    /// # Example:
    ///
    /// ```ignore
    /// let name = Name::new(b"Name");
    /// // '/Name' has length of 5 bytes.
    /// assert_eq!(name.len(), 5); //
    /// ```
    pub(crate) const fn len(&self) -> usize {
        self.0.len() + 1
    }
}

impl WriteDictValue for Name {
    fn write(&self, writer: &mut impl Write) -> Result<usize, Error> {
        self.write(writer)
    }
}

#[cfg(test)]
mod tests {
    use super::Name;

    #[test]
    pub fn new_name() {
        const PDF_KEY: Name = Name::new(b"PdfKey");

        let mut out_buf = Vec::new();
        PDF_KEY.write(&mut out_buf).unwrap();
        assert_eq!(&out_buf, b"/PdfKey ");
    }
}
