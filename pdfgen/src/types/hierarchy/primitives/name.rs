//! Implementation of PDF name.

use std::io::{Error, Write};

use pdfgen_macros::const_names;

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
#[derive(Debug, Clone)]
pub(crate) struct Name<T: AsRef<[u8]>> {
    inner: T,
}

pub(crate) type OwnedName = Name<Vec<u8>>;

impl<T: AsRef<[u8]>> Name<T> {
    /// Creates a new [`Name`] from a value implementing `AsRef<[u8]>`.
    pub fn new(inner: T) -> Self {
        let inner_ref = inner.as_ref();
        if inner_ref.is_empty() {
            panic!("Dictionary Key must start with '/' followed by at least one ASCII character.");
        }

        if inner_ref.contains(&b'/') {
            panic!("Dictionary Key is not allowed to contain '/'.");
        }

        Self { inner }
    }

    /// Encode and write this `Name` into the provided implementor of [`Write`].
    pub fn write(&self, writer: &mut dyn Write) -> Result<usize, Error> {
        Ok(pdfgen_macros::write_chain! {
            writer.write(b"/"),
            writer.write(self.inner.as_ref()),
            writer.write(b" "),
        })
    }

    /// The number of bytes that this `Name` occupies when written into the PDF document. This does
    /// not include the whitespace written after the `Name`.
    pub fn len(&self) -> usize {
        self.inner.as_ref().len() + 1
    }

    /// Returns the referenced version to this `Name`.
    pub fn as_ref(&self) -> Name<&[u8]> {
        Name {
            inner: self.inner.as_ref(),
        }
    }

    /// Returns the inner byte slice
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(self.inner.as_ref().len() + 2);
        let _ = self.write(&mut vec);

        vec
    }
}

impl Name<&'static [u8]> {
    const_names!(pub(crate) TYPE);

    /// Create a new [`Name`] from a static byte slice.
    /// This allows seamless creation of `Name` for static data without specifying lifetimes.
    pub const fn from_static(inner: &'static [u8]) -> Self {
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

        Self { inner }
    }
}

#[cfg(test)]
mod tests {
    use super::Name;

    #[test]
    pub fn new_name_static() {
        let static_key = Name::from_static(b"StaticKey");
        const STATIC_KEY: Name<&'static [u8]> = Name::from_static(b"StaticKey");

        let mut out_buf = Vec::new();
        static_key.write(&mut out_buf).unwrap();
        assert_eq!(&out_buf, b"/StaticKey ");
        out_buf.clear();

        STATIC_KEY.write(&mut out_buf).unwrap();
        assert_eq!(&out_buf, b"/StaticKey ");
    }

    #[test]
    pub fn new_name_dynamic() {
        let dynamic_key = Name::new(Vec::from("DynamicKey"));

        let mut out_buf = Vec::new();
        dynamic_key.write(&mut out_buf).unwrap();
        assert_eq!(&out_buf, b"/DynamicKey ");
    }

    #[test]
    pub fn new_name_slice() {
        let slice_key = Name::new(b"SliceKey".as_ref());

        let mut out_buf = Vec::new();
        slice_key.write(&mut out_buf).unwrap();
        assert_eq!(&out_buf, b"/SliceKey ");
    }
}
