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

#[derive(Clone, Copy, PartialEq, Eq, Debug, thiserror::Error)]
pub enum ParseIdentifierErr {
    #[error("Identifier is not allowed to start with a solidus '/'.")]
    StartsWithSolidus,

    #[error("Identifier must not contain the NULL character.")]
    ContainsNull,
}

impl FromStr for Identifier<String> {
    type Err = ParseIdentifierErr;

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

        if input.starts_with('/') {
            return Err(ParseIdentifierErr::StartsWithSolidus);
        }

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
                b'\0' => return Err(ParseIdentifierErr::ContainsNull),
                b'#' => output.push_str("#23"),
                to_encode if !(0x21..=0x7e).contains(&to_encode) || is_delimiter(to_encode) => {
                    write!(&mut output, "#{to_encode:x}")
                        .expect("Writing to String should always succeed.");
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
        use crate::types::hierarchy::primitives::identifier::ParseIdentifierErr;

        use super::super::Identifier;
        use std::str::FromStr;
        macro_rules! quoted_identest {
            ($name:expr, @$expected:literal) => {
                let name = Identifier::from_str($name).expect("Could not parse name.");
                let mut out_buf = vec![b'\''];
                name.write(&mut out_buf)
                    .expect("Could not write to output buffer.");
                out_buf.push(b'\'');
                let out_string = String::from_utf8(out_buf).expect("output buffer not valid UTF8.");
                insta::assert_snapshot!(out_string, @$expected);
            };
        }

        #[test]
        fn starts_with_solidus() {
            let ident_res = Identifier::from_str("/InvalidIdent");

            assert!(matches!(
                ident_res,
                Err(ParseIdentifierErr::StartsWithSolidus)
            ));
        }

        #[test]
        fn contains_null() {
            let ident_res = Identifier::from_str("Contains\0Null");
            assert!(matches!(ident_res, Err(ParseIdentifierErr::ContainsNull)));
        }

        #[test]
        fn whitespace() {
            quoted_identest!("This is Name.", @r"'/This#20is#20Name. '");
            quoted_identest!("Trailing   ", @r"'/Trailing#20#20#20 '");
        }

        #[test]
        fn regular_chars() {
            quoted_identest!("ThisName", @r"'/ThisName '");
            quoted_identest!("AnotherName", @r"'/AnotherName '");
            quoted_identest!("Some-weird_cha'rs", @r"'/Some-weird_cha'rs '");
        }

        #[test]
        fn delimiters() {
            quoted_identest!("This()Name", @r"'/This#28#29Name '");
            quoted_identest!("This<>Name", @r"'/This#3c#3eName '");
            quoted_identest!("This[]Name", @r"'/This#5b#5dName '");
            quoted_identest!("This{}Name", @r"'/This#7b#7dName '");
            quoted_identest!("This/Name", @r"'/This#2fName '");
            quoted_identest!("This%Name", @r"'/This#25Name '");
        }
    }
}
