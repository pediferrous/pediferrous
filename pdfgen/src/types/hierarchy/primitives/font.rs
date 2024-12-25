//! Comment

use crate::types::{self, constants};

use super::{name::Name, obj_id::ObjId, object::Object};

/// Comment
pub struct Font<const N: usize> {
    /// Comment
    id: ObjId,

    /// Comment
    subtype: &'static [u8; N],

    /// Comment
    base_type: &'static [u8; N],
}

impl<const N: usize> Font<N> {
    const FONT: Name = Name::new(b"Font");
    const SUBTYPE: Name = Name::new(b"Subtype");
    const BASE_FONT: Name = Name::new(b"BaseFont");

    /// Comment
    pub fn new(id: ObjId, subtype: &'static [u8; N], base_type: &'static [u8; N]) -> Self {
        Font {
            id,
            subtype,
            base_type,
        }
    }

    /// Returns the object reference of this Font object.
    pub fn obj_ref(&self) -> ObjId {
        self.id.clone()
    }
}

impl<const N: usize> Object for Font<N> {
    fn write(&self, writer: &mut impl std::io::Write) -> Result<usize, std::io::Error> {
        let subtype_value: Name = Name::new(self.subtype);
        let base_type_value: Name = Name::new(self.base_type);

        let bytes_written = types::write_chain! {
            writer.write(b"<< "),

            // /Type /Font
            Name::TYPE.write(writer),
            Self::FONT.write(writer),
            writer.write(constants::NL_MARKER),

            // /Subtype /xyz
            Self::SUBTYPE.write(writer),
            subtype_value.write(writer),
            writer.write(constants::NL_MARKER),

            // /BaseFont /xyz
            Self::BASE_FONT.write(writer),
            base_type_value.write(writer),
            writer.write(constants::NL_MARKER),

            writer.write(b">>"),
        };

        Ok(bytes_written)
    }
}
