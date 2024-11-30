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

    pub fn new(parent: impl Into<String>, media_box: impl Into<String>) -> Self {
        Self {
            parent: parent.into(),
            resources: String::from("<< >>"),
            media_box: media_box.into(),
        }
    }
}

impl Page {
    pub fn write(&self, writer: &mut impl Write) -> Result<usize, Error> {
        let written = types::write_chain! {
            writer.write(b"<< "),
            Name::TYPE.write(writer),
            Self::TYPE.write(writer),

            Self::PARENT.write(writer),
            writer.write(self.parent.as_bytes()),
            writer.write(b" "),

            Self::RESOURCES.write(writer),
            writer.write(self.resources.as_bytes()),
            writer.write(b" "),

            Self::MEDIA_BOX.write(writer),
            writer.write(self.media_box.as_bytes()),
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
        let page = Page::new("0 0 R", "[0 0 100 100]");

        let mut writer = Vec::new();
        page.write(&mut writer).unwrap();

        let output = String::from_utf8(writer).unwrap();

        insta::assert_snapshot!(
            output,
            @"<< /Type /Page /Parent 0 0 R /Resources << >> /MediaBox [0 0 100 100] >>"
        );
    }
}
