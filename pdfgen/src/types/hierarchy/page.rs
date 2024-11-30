use std::io::{Error, Write};

use crate::types;

use super::primitives::{name::Name, obj_ref::ObjRef, rectangle::Rectangle};

pub struct Page {
    parent: ObjRef,
    resources: String,
    media_box: Rectangle,
}

impl Page {
    const TYPE: Name = Name::new(b"Page");
    const PARENT: Name = Name::new(b"Parent");
    const RESOURCES: Name = Name::new(b"Resources");
    const MEDIA_BOX: Name = Name::new(b"MediaBox");

    pub fn new(parent: impl Into<ObjRef>, media_box: impl Into<Rectangle>) -> Self {
        Self {
            parent: parent.into(),
            resources: String::from("<< >>"),
            media_box: media_box.into(),
        }
    }

    pub fn write(&self, writer: &mut impl Write) -> Result<usize, Error> {
        let written = types::write_chain! {
            writer.write(b"<< "),
            Name::TYPE.write(writer),
            Self::TYPE.write(writer),

            Self::PARENT.write(writer),
            self.parent.write(writer),
            writer.write(b" "),

            Self::RESOURCES.write(writer),
            writer.write(self.resources.as_bytes()),
            writer.write(b" "),

            Self::MEDIA_BOX.write(writer),
            self.media_box.write(writer),
            writer.write(b" >>"),
        };

        Ok(written)
    }
}

#[cfg(test)]
mod tests {
    use super::Page;

    #[test]
    fn basic_page() {
        let page = Page::new(0, (0, 0, 100, 100));

        let mut writer = Vec::new();
        page.write(&mut writer).unwrap();

        let output = String::from_utf8(writer).unwrap();

        insta::assert_snapshot!(
            output,
            @"<< /Type /Page /Parent 0 0 R /Resources << >> /MediaBox [0 0 100 100] >>"
        );
    }
}
