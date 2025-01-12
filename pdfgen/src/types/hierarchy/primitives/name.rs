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
pub(crate) struct Name<'a>(&'a [u8]);

impl<'a> Name<'a> {
    /// Convenience constant for commonly used names with static data.
    pub(crate) const TYPE: Name<'static> = Name::from_static(b"Type");

    /// Create a new [`Name`] from a byte slice with a lifetime.
    /// This is the main constructor for dynamic data.
    pub(crate) fn new(inner: &'a [u8]) -> Self {
        if inner.is_empty() {
            panic!("Dictionary Key must start with '/' followed by at least one ASCII character.");
        }

        if inner.contains(&b'/') {
            panic!("Dictionary Key is not allowed to contain '/'.");
        }

        Self(inner)
    }

    /// Create a new [`Name`] from a static byte slice.
    /// This allows seamless creation of `Name` for static data without specifying lifetimes.
    pub(crate) const fn from_static(inner: &'static [u8]) -> Name<'static> {
        if inner.is_empty() {
            panic!("Dictionary Key must start with '/' followed by at least one ASCII character.");
        }

        let mut i = 0;
        while i < inner.len() {
            if inner[i] == b'/' {
                panic!("Dictionary Key is not allowed to contain '/'.");
            }
            i += 1;
        }

        Name(inner)
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
    /// ```
    /// let name = Name::new(b"Name");
    /// // '/Name' has length of 5 bytes.
    /// assert_eq!(name.len(), 5); //
    /// ```
    pub(crate) fn len(&self) -> usize {
        self.0.len() + 1
    }
}

impl WriteDictValue for Name<'_> {
    fn write(&self, writer: &mut impl Write) -> Result<usize, Error> {
        self.write(writer)
    }
}

#[cfg(test)]
mod tests {
    use super::Name;

    #[test]
    pub fn new_name_static() {
        const PDF_KEY: Name = Name::from_static(b"PdfKey");

        let mut out_buf = Vec::new();
        PDF_KEY.write(&mut out_buf).unwrap();
        assert_eq!(&out_buf, b"/PdfKey ");
    }

    #[test]
    pub fn new_name_dynamic() {
        let pdf_key = Name::new(b"DynamicKey");

        let mut out_buf = Vec::new();
        pdf_key.write(&mut out_buf).unwrap();
        assert_eq!(&out_buf, b"/DynamicKey ");
    }
}
