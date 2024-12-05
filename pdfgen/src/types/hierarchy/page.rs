use std::io::{Error, Write};

use crate::types;

use super::primitives::{
    name::Name, obj_id::ObjId, object::Object, rectangle::Rectangle, resources::Resources,
};

/// Page objects are the leaves of the page tree, each of which is a dictionary specifying the
/// attributes of a single page of the document.
pub struct Page {
    /// The page tree node that is the immediate parent of this page object.
    parent: ObjId,

    /// A dictionary containing any resources required by the page contents. If the page requires
    /// no resources, the value of this entry shall be an empty dictionary.
    resources: Resources,

    /// A [`Rectangle`], expressed in default user space units, that shall define the boundaries of
    /// the physical medium on which the page shall be displayed or printed.
    media_box: Rectangle,
}

impl Page {
    const TYPE: Name = Name::new(b"Page");
    const PARENT: Name = Name::new(b"Parent");
    const RESOURCES: Name = Name::new(b"Resources");
    const MEDIA_BOX: Name = Name::new(b"MediaBox");

    /// Create a new blank page that belongs to the given parent and media box.
    pub fn new(parent: ObjId, media_box: impl Into<Rectangle>) -> Self {
        Self {
            parent,
            resources: Resources::default(),
            media_box: media_box.into(),
        }
    }
}

impl Object for Page {
    /// Encode the PDF Page into the given implementor of [`Write`].
    fn write(&self, writer: &mut impl Write) -> Result<usize, Error> {
        let written = types::write_chain! {
            writer.write(b"<< "),
            Name::TYPE.write(writer),
            Self::TYPE.write(writer),

            Self::PARENT.write(writer),
            self.parent.write_ref(writer),
            writer.write(b" "),

            Self::RESOURCES.write(writer),
            self.resources.write(writer),
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
    use crate::types::hierarchy::primitives::{obj_id::IdManager, object::Object};

    #[test]
    fn basic_page() {
        let mut id_manager = IdManager::default();
        let page = Page::new(id_manager.create_id(), (0, 0, 100, 100));

        let mut writer = Vec::new();
        page.write(&mut writer).unwrap();

        let output = String::from_utf8(writer).unwrap();

        insta::assert_snapshot!(
            output,
            @"<< /Type /Page /Parent 0 0 R /Resources <<  >> /MediaBox [0 0 100 100] >>"
        );
    }
}
