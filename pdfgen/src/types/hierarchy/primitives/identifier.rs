//! Implementation of PDF name.

use std::{
    io::{Error, Write},
    str::FromStr,
};

use pdfgen_macros::const_identifiers;

/// [`Identifier`] refers to the `Name` object in PDF and it is an atomic symbol uniquely defined
/// by a sequence of any characters (8-bit values) except null (character code 0) that follow these
/// rules:
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
/// All characters except the white-space characters and delimiters are referred to as regular
/// characters.
///
/// From Section 7.2.3, Table 2 delimiters are:
///
/// | Glyph | Decimal | Hexadecimal | Octal | Name                 |
/// |-------|---------|-------------|-------|----------------------|
/// |   (   |     40  |        28   |  050  | LEFT PARENTHESIS     |
/// |   )   |     41  |        29   |  051  | RIGHT PARENTHESIS    |
/// |   <   |     60  |        3C   |  074  | LESS-THAN SIGN       |
/// |   >   |     62  |        3E   |  076  | GREATER-THAN SIGN    |
/// |   [   |     91  |        5B   |  133  | LEFT SQUARE BRACKET  |
/// |   ]   |     93  |        5D   |  135  | RIGHT SQUARE BRACKET |
/// |   {   |    123  |        7B   |  173  | LEFT CURLY BRACKET   |
/// |   }   |    125  |        7D   |  175  | RIGHT CURLY BRACKET  |
/// |   /   |     47  |        2F   |  057  | SOLIDUS              |
/// |   %   |     37  |        25   |  045  | PERCENT SIGN         |
///
#[derive(Debug, Clone)]
pub struct Identifier<T: AsRef<[u8]>> {
    inner: T,
}

pub(crate) type OwnedIdentifier = Identifier<Vec<u8>>;

impl<T: AsRef<[u8]>> Identifier<T> {
    /// Creates a new [`Identifier`] from a value implementing `AsRef<[u8]>`.
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

    /// Encode and write this [`Identifier`] into the provided implementor of [`Write`].
    pub fn write(&self, writer: &mut dyn Write) -> Result<usize, Error> {
        Ok(pdfgen_macros::write_chain! {
            writer.write(b"/"),
            writer.write(self.inner.as_ref()),
            writer.write(b" "),
        })
    }

    /// The number of bytes that this [`Identifier`] occupies when written into the PDF document. This does
    /// not include the whitespace written after the [`Identifier`].
    // NOTE(nfejzic): empty `Identifier` is not valid, so we don't need `is_empty` method.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.inner.as_ref().len() + 1
    }

    /// Returns the referenced version to this [`Identifier`].
    pub fn as_ref(&self) -> Identifier<&[u8]> {
        Identifier {
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

impl Identifier<&'static [u8]> {
    const_identifiers! {
        pub(crate) TYPE,
        pub(crate) X_OBJECT,
        pub(crate) FONT
    }

    /// Create a new [`Identifier`] from a static byte slice.
    /// This allows seamless creation of [`Identifier`] for static data without specifying lifetimes.
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

impl FromStr for Identifier<String> {
    type Err = String;

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
    /// All characters except the white-space characters and delimiters are referred to as regular
    /// characters.
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use std::fmt::Write;

        let input = input.trim_start();
        let mut output = String::with_capacity(input.len());

        fn is_delimiter(byte: u8) -> bool {
            byte == 40
                || byte == 41
                || byte == 60
                || byte == 62
                || byte == 91
                || byte == 93
                || byte == 123
                || byte == 125
                || byte == 47
                || byte == 37
        }

        for ch in input.bytes() {
            match ch {
                b'#' => output.push_str("#23"),
                to_encode if !(0x21..=0x7e).contains(&to_encode) || is_delimiter(to_encode) => {
                    write!(&mut output, "#{to_encode:x}")
                        .map_err(|_| String::from("Failed writing to string."))?;
                }
                inside_range => {
                    output.push(inside_range as char);
                }
            };
        }

        Ok(Identifier::new(output))
    }
}

#[cfg(test)]
mod tests {
    use super::Identifier;

    #[test]
    pub fn new_name_static() {
        let static_key = Identifier::from_static(b"StaticKey");
        const STATIC_KEY: Identifier<&'static [u8]> = Identifier::from_static(b"StaticKey");

        let mut out_buf = Vec::new();
        static_key.write(&mut out_buf).unwrap();
        assert_eq!(&out_buf, b"/StaticKey ");
        out_buf.clear();

        STATIC_KEY.write(&mut out_buf).unwrap();
        assert_eq!(&out_buf, b"/StaticKey ");
    }

    #[test]
    pub fn new_name_dynamic() {
        let dynamic_key = Identifier::new(Vec::from("DynamicKey"));

        let mut out_buf = Vec::new();
        dynamic_key.write(&mut out_buf).unwrap();
        assert_eq!(&out_buf, b"/DynamicKey ");
    }

    #[test]
    pub fn new_name_slice() {
        let slice_key = Identifier::new(b"SliceKey".as_ref());

        let mut out_buf = Vec::new();
        slice_key.write(&mut out_buf).unwrap();
        assert_eq!(&out_buf, b"/SliceKey ");
    }

    mod parsing {
        macro_rules! identest {
            ($name:expr, $expected:expr) => {{
                use super::super::Identifier;
                use std::str::FromStr;

                let name = Identifier::from_str($name).expect("Could not parse name.");
                let mut out_buf = Vec::new();
                name.write(&mut out_buf)
                    .expect("Could not write to output buffer.");
                let out_string = String::from_utf8(out_buf).expect("output buffer not valid UTF8.");
                assert_eq!(out_string, $expected);
            }};
        }

        #[test]
        fn parse_whitespace() {
            identest!("This is Name.", "/This#20is#20Name. ");
        }

        #[test]
        fn parse_regular_chars() {
            identest!("ThisName", "/ThisName ");
        }

        #[test]
        fn parse_parentheses() {
            identest!("This()Name", "/This#28#29Name ");
        }
    }
}
