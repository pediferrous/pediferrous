//! Implementation of PDF Text object.

use super::{font::Font, object::Object, rectangle::Position, string::PdfString};

/// Comment
#[derive(Debug)]
pub struct TextTransform {
    /// Comment
    position: Position,

    /// Comment
    size: u32,
}

/// Comment
#[derive(Debug)]
pub struct Text {
    /// Comment
    content: PdfString,

    /// Comment
    transform: TextTransform,

    /// Comment
    font: Font,
}

impl Text {
    /// Comment
    fn new(content: &str) -> Self {
        Self { content: content, transform TextTransform::default(), font: Font::default() }
    }
}
/// Comment
pub struct TextBuilder<const IS_INIT: bool> {
    /// Comment
    inner: Text,
}

impl<const IS_INIT: bool> TextBuilder<IS_INIT> {
    /// Sets the position of the [`Text`] on a page.
    pub fn at(mut self, pos: Position) -> TextBuilder<true> {
        self.inner.transform.position = pos;
        TextBuilder { inner: self.inner }
    }

    /// Sets the scaling of the image to the given width and height.
    pub fn with_size(mut self, size: u32) -> Self {
        self.inner.transform.size = size;
        self
    }
}

impl TextBuilder<true> {
    pub fn build(self) -> Text {
        self.inner
    }
}
