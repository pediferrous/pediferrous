use std::io::{Error, Write};

use crate::types;

use super::primitives::name::Name;

pub struct Page {
    parent: String,
    resources: String,
    media_box: String,
}

impl Page {
    const TYPE: Name = Name::new(b"Page");
    const PARENT: Name = Name::new(b"Parent");
    const RESOURCES: Name = Name::new(b"Resources");
    const MEDIA_BOX: Name = Name::new(b"MediaBox");
}

impl Page {
    pub fn write(&self, writer: &mut impl Write) -> Result<usize, Error> {
        let written = types::write_chain! {
            writer.write(b"<< "),
            Name::TYPE.write(writer),
            Self::TYPE.write(writer),
            writer.write(b"\n"),
            Self::PARENT.write(writer),
            writer.write(self.parent.as_bytes()),
            Self::RESOURCES.write(writer),
            writer.write(self.resources.as_bytes()),
            Self::MEDIA_BOX.write(writer),
            writer.write(self.media_box.as_bytes()),
            writer.write(b" >>"),
        };

        Ok(written)
    }
}
