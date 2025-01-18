use std::{
    io::{Error, Write},
    ops::Not,
};

use pdfgen_macros::const_names;

use crate::types::{self, constants};

use super::{
    content::{image::ImageTransform, ContentStream, Operation},
    primitives::{
        name::Name, obj_id::ObjId, object::Object, rectangle::Rectangle, resources::Resources,
    },
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

    /// Content stream holds the encoded bytes with various contents added to the page.
    contents: ContentStream,
}

impl Page {
    const_names! {
        PAGE,
        PARENT,
        RESOURCES,
        MEDIA_BOX,
        CONTENTS,
    }

    /// Create a new blank page that belongs to the given parent and media box.
    pub fn new(id: ObjId, contents_id: ObjId, parent: ObjId) -> Self {
        Self {
            id,
            parent,
            resources: Resources::default(),
            media_box: None,
            contents: ContentStream::new(contents_id),
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

    pub fn add_image(&mut self, img_ref: ObjId, transform: ImageTransform) {
        let name = self.resources.add_image(img_ref);
        self.contents
            .add_content(Operation::DrawImage { name, transform });
    }

    pub(crate) fn content_stream(&self) -> &ContentStream {
        &self.contents
    }
}

impl Object for Page {
    /// Encode the PDF Page into the given implementor of [`Write`].
    fn write(&self, writer: &mut dyn Write) -> Result<usize, Error> {
        let written = types::write_chain! {
            writer.write(b"<< "),
            Name::TYPE.write(writer),
            Self::PAGE.write(writer),
            writer.write(constants::NL_MARKER),

            Self::PARENT.write(writer),
            self.parent.write_ref(writer),
            writer.write(constants::NL_MARKER),

            Self::RESOURCES.write(writer),
            self.resources.write(writer),
            writer.write(constants::NL_MARKER),

            self.media_box.map(|rect| Self::write_mediabox(writer, rect)).unwrap_or(Ok(0)),

            self.contents
                .is_empty()
                .not()
                .then_some(())
                .map(|_| {
                    Ok::<usize, Error>(types::write_chain! {
                        Self::CONTENTS.write(writer),
                        self.contents.obj_ref().write_ref(writer),
                        writer.write(constants::NL_MARKER),
                    })
                })
                .unwrap_or(Ok(0)),

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
        let mut page = Page::new(
            id_manager.create_id(),
            id_manager.create_id(),
            id_manager.create_id(),
        );
        page.set_mediabox(Rectangle::from_units(0.0, 0.0, 100.0, 100.0));

        let mut writer = Vec::new();
        page.write(&mut writer).unwrap();

        let output = String::from_utf8(writer).unwrap();

        insta::assert_snapshot!(
            output,
            @r"
        << /Type /Page 
        /Parent 3 0 R
        /Resources <<  >>
        /MediaBox [0 0 100 100] >>
        "
        );
    }
}
