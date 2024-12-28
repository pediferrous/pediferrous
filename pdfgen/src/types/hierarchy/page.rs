use std::io::{Error, Write};

use crate::types::{self};

use super::primitives::{
    name::Name, obj_id::ObjId, object::Object, rectangle::Rectangle, resources::Resources,
};

/// Page objects are the leaves of the page tree, each of which is a dictionary specifying the
/// attributes of a single page of the document.
pub struct Page {
    /// ID of this Page object.
    id: ObjId,

    /// The page tree node that is the immediate parent of this page object.
    parent: ObjId,

    /// A dictionary containing any resources required by the page contents. If the page requires
    /// no resources, the value of this entry shall be an empty dictionary.
    resources: Resources,

    /// A [`Rectangle`], expressed in default user space units, that shall define the boundaries of
    /// the physical medium on which the page shall be displayed or printed.
    media_box: Option<Rectangle>,
}

impl Page {
    const TYPE: Name = Name::new(b"Page");
    const PARENT: Name = Name::new(b"Parent");
    const RESOURCES: Name = Name::new(b"Resources");
    const MEDIA_BOX: Name = Name::new(b"MediaBox");

    /// Create a new blank page that belongs to the given parent and media box.
    pub fn new(id: ObjId, parent: ObjId) -> Self {
        Self {
            id,
            parent,
            resources: Resources::default(),
            media_box: None,
        }
    }

    pub fn set_mediabox(&mut self, media_box: impl Into<Rectangle>) {
        self.media_box = Some(media_box.into());
    }

    /// Returns the object reference of this Page object.
    pub fn obj_ref(&self) -> ObjId {
        self.id.clone()
    }

    fn write_mediabox(writer: &mut dyn Write, rect: Rectangle) -> Result<usize, Error> {
        Ok(types::write_chain! {
            Self::MEDIA_BOX.write(writer),
            rect.write(writer),
        })
    }

    pub fn add_image(&mut self, img_ref: ObjId) {
        self.resources.add_image(img_ref);
    }
}

impl Object for Page {
    /// Encode the PDF Page into the given implementor of [`Write`].
    fn write(&self, writer: &mut dyn Write) -> Result<usize, Error> {
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

            self.media_box.map(|rect| Self::write_mediabox(writer, rect)).unwrap_or(Ok(0)),
            writer.write(b" >>"),
        };

        Ok(written)
    }

    fn obj_ref(&self) -> &ObjId {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::Page;
    use crate::types::hierarchy::primitives::{
        obj_id::IdManager, object::Object, rectangle::Rectangle,
    };

    #[test]
    fn basic_page() {
        let mut id_manager = IdManager::default();
        let mut page = Page::new(id_manager.create_id(), id_manager.create_id());
        page.set_mediabox(Rectangle::from_units(0.0, 0.0, 100.0, 100.0));

        let mut writer = Vec::new();
        page.write(&mut writer).unwrap();

        let output = String::from_utf8(writer).unwrap();

        insta::assert_snapshot!(
            output,
            @"<< /Type /Page /Parent 1 0 R /Resources <<  >> /MediaBox [0 0 100 100] >>"
        );
    }
}
