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
