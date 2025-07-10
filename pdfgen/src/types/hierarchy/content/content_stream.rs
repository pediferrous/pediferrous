use crate::{
    ObjId,
    types::{
        constants,
        hierarchy::primitives::{name::Name, object::Object, rectangle::Position},
    },
};

use super::{image::ImageTransform, stream::Stream, text::Text};

/// Represents a specific operation in [`ContentStream`] such as drawing an image or text.
pub(crate) enum Operation<'a> {
    /// Represents a image drawing operation.
    DrawImage {
        /// Name of the [`Image`] as defined in [`Resources`] of a [`Page`].
        ///
        /// [`Image`]: super::image::Image
        /// [`Resources`]: crate::types::hierarchy::primitives::resources::Resources
        /// [`Page`]: crate::types::hierarchy::page::Page
        name: Name<&'a [u8]>,

        /// Transformation that should be applied to the [`Image`] in Pdf.
        ///
        /// [`Image`]: super::image::Image
        transform: ImageTransform,
    },

    /// Represents a text drawing operation.
    DrawText {
        /// Text object to be drawn.
        text: Text,

        /// Name of the [`Font`] as defined in [`Resources`] of a [`Page`].
        ///
        /// [`Font`]: crate::types::hierarchy::primitives::font::Font
        /// [`Resources`]: crate::types::hierarchy::primitives::resources::Resources
        /// [`Page`]: crate::types::hierarchy::page::Page
        font_name: Name<&'a [u8]>,
    },
}

/// Represents the content stream object that is used for encoding and rendering content of a
/// [`Page`].
///
/// [`Page`]: crate::types::hierarchy::page::Page
#[derive(Debug, PartialEq, PartialOrd)]
pub struct ContentStream {
    id: ObjId,

    /// Inner stream object containing the actual bytes of the content.
    stream: Stream,
}

impl ContentStream {
    /// Creates a new `ContentStream` with the given [`ObjId`].
    pub fn new(id: ObjId) -> Self {
        Self {
            id,
            stream: Stream::new(),
        }
    }

    /// Adds a content to this `ContentStream` that should be displayed on a [`Page`]. Content is
    /// added in means of `Operation` that describes specific content elements.
    pub(crate) fn add_content(&mut self, operation: Operation) {
        match operation {
            Operation::DrawImage { name, transform } => self.draw_image(name, transform),
            Operation::DrawText { text, font_name } => self.draw_text(text, font_name),
        }
    }

    /// Encodes an image in this `ContentStream`.
    fn draw_image(&mut self, name: Name<&[u8]>, transform: ImageTransform) {
        let Position {
            x: width,
            y: height,
        } = transform.scale;

        let Position { x, y } = transform.position;

        // Save graphics state
        self.stream.push_bytes(b"q");
        self.stream.push_bytes(constants::NL_MARKER);

        // apply transform 🤯
        // width 0 0 height x y cm - Translate to (x, y) and scale to width x height
        self.stream
            .push_bytes(format!("{width} 0 0 {height} {x} {y} cm").as_bytes());
        self.stream.push_bytes(constants::NL_MARKER);

        // /ImgName Do - Paint image
        self.stream.push_bytes(&name.to_bytes());
        self.stream.push_bytes(b"Do");
        self.stream.push_bytes(constants::NL_MARKER);

        // Restore graphics state
        self.stream.push_bytes(b"Q");
    }

    /// Encodes a text object in this `ContentStream`.
    fn draw_text(&mut self, text: Text, font_name: Name<&[u8]>) {
        self.stream.push_bytes(
            &text
                .to_bytes(font_name)
                .expect("Writing to Vec should never fail."),
        );
    }

    pub fn is_empty(&self) -> bool {
        self.stream.is_empty()
    }

    pub(crate) fn obj_ref(&self) -> &ObjId {
        &self.id
    }
}

impl Object for ContentStream {
    fn write_def(&self, writer: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        Ok(pdfgen_macros::write_chain! {
            self.id.write_def(writer),
            writer.write(constants::NL_MARKER),
        })
    }

    fn write_content(&self, writer: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        Ok(pdfgen_macros::write_chain! {
            self.stream.write(writer),
            writer.write(constants::NL_MARKER),
        })
    }
}
