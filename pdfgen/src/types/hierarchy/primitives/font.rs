//! Comment

use crate::types::{self, constants};

use super::{name::Name, obj_id::ObjId, object::Object};

/// Comment
pub struct Font {
    /// Comment
    id: ObjId,

    /// Comment
    subtype: Vec<u8>,

    /// Comment
    base_type: Vec<u8>,
}

impl Font {
    const FONT: Name<'static> = Name::from_static(b"Font");
    const SUBTYPE: Name<'static> = Name::from_static(b"Subtype");
    const BASE_FONT: Name<'static> = Name::from_static(b"BaseFont");

    /// Comment
    pub fn new(id: ObjId, subtype: Vec<u8>, base_type: Vec<u8>) -> Self {
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

impl Object for Font {
    fn write(&self, writer: &mut impl std::io::Write) -> Result<usize, std::io::Error> {
        let subtype_value: Name = Name::new(&self.subtype);
        let base_type_value: Name = Name::new(&self.base_type);

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
