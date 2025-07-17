use std::io::{Error, Write};

use pdfgen_macros::const_identifiers;

use crate::{IdManager, ObjId, types::constants};

use super::{
    content::{ContentStream, Operation, image::Image, text::Text},
    page_tree::PageTree,
    primitives::{font::Font, identifier::Identifier, rectangle::Rectangle, resources::Resources},
};

/// Page objects are the leaves of the page tree, each of which is a dictionary specifying the
/// attributes of a single page of the document.
pub struct Page {
    /// ID of this Page object.
    id: ObjId<Self>,

    /// The page tree node that is the immediate parent of this page object.
    parent: ObjId<PageTree>,

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
    const_identifiers! {
        PAGE,
        PARENT,
        RESOURCES,
        MEDIA_BOX,
        CONTENTS,
    }

    /// Create a new blank page that belongs to the given parent and media box.
    pub fn new(
        id: ObjId<Self>,
        contents_id: ObjId<ContentStream>,
        parent: ObjId<PageTree>,
    ) -> Self {
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
    pub fn obj_ref(&self) -> ObjId<Self> {
        self.id.clone()
    }

    fn write_mediabox(writer: &mut dyn Write, rect: Rectangle) -> Result<usize, Error> {
        Ok(pdfgen_macros::write_chain! {
            Self::MEDIA_BOX.write(writer),
            rect.write(writer),
        })
    }

    // ids = vec[17, 18]
    // add_image -> id = ids.len() = 0; ids.push(0);
    // add_image -> id = ids.len() = 1; ids.push(0);
    //
    // when rendering:
    // for each id in ids.iter_mut() { *id = id_manager.create_id() }
    pub fn add_image(&mut self, image: Image) {
        // /Im1 <-> ids[0] -> /Im1 17
        // ids[0] obj    -> 17 0 obj
        let transform = image.transform();
        let name = self.resources.add_image(image);

        self.contents
            .add_content(Operation::DrawImage { name, transform });
    }

    /// Adds a text to the PDF page.
    pub fn add_text(&mut self, text: Text, font_id: ObjId<Font>) {
        let font_name = self.resources.add_font(font_id);

        self.contents
            .add_content(Operation::DrawText { text, font_name });
    }

    pub(crate) fn content_stream(&self) -> &ContentStream {
        &self.contents
    }

    /// Encode the PDF Page into the given implementor of [`Write`].
    pub(crate) fn write(
        &self,
        writer: &mut dyn Write,
        id_manager: &mut IdManager,
    ) -> Result<(usize, Vec<usize>), Error> {
        let mut offsets = Vec::with_capacity(self.resources.entries.len());

        let mut renderable_resources = self.resources.renderables(id_manager);

        let written = pdfgen_macros::write_chain! {
            self.id.write_def(writer),
            writer.write(constants::NL_MARKER),

            writer.write(b"<< "),
            Identifier::TYPE.write(writer),
            Self::PAGE.write(writer),
            writer.write(constants::NL_MARKER),

            Self::PARENT.write(writer),
            self.parent.write_ref(writer),
            writer.write(constants::NL_MARKER),

            Self::RESOURCES.write(writer),
            self.resources.write_dict(writer, &renderable_resources),
            writer.write(constants::NL_MARKER),

            if let Some(media_box) = self.media_box {
                Self::write_mediabox(writer, media_box),
            },

            if !self.contents.is_empty() {
                Self::CONTENTS.write(writer),
                self.contents.obj_ref().write_ref(writer),
                writer.write(constants::NL_MARKER),
            },

            writer.write(b">>"),
            writer.write(constants::NL_MARKER),

            // endobj\n
            writer.write(constants::END_OBJ_MARKER),
            writer.write(constants::NL_MARKER),
            writer.write(constants::NL_MARKER),

            for renderable_entry in renderable_resources.iter_mut() {
                {
                    offsets.push(written);
                    renderable_entry.write_def(writer)
                }
            },

            writer.write(constants::NL_MARKER),
        };

        Ok((written, offsets))
    }
}

#[cfg(test)]
mod tests {
    use super::Page;
    use crate::{IdManager, types::hierarchy::primitives::rectangle::Rectangle};

    #[test]
    fn basic_page() {
        let mut id_manager = IdManager::new();
        let mut page = Page::new(
            id_manager.create_id(),
            id_manager.create_id(),
            id_manager.create_id(),
        );
        page.set_mediabox(Rectangle::from_units(0.0, 0.0, 100.0, 100.0));

        let mut writer = Vec::new();
        page.write(&mut writer, &mut id_manager).unwrap();

        let output = String::from_utf8(writer).unwrap();

        insta::assert_snapshot!(
            output,
            @r"
        1 0 obj
        << /Type /Page 
        /Parent 3 0 R
        /Resources <<  >>
        /MediaBox [0 0 100 100]>>
        endobj
        "
        );
    }
}
