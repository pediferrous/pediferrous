//! Implementation of PDF Text object.

use std::io::{self, Write};

use crate::types::{
    constants,
    hierarchy::primitives::{identifier::Identifier, rectangle::Position, string::PdfString},
};

use super::color::Color;

/// Defines the transformation properties of a [`Text`] object, including its position and size on a [`Page`].
///
/// [`Page`]: crate::types::hierarchy::page::Page
#[derive(Debug, Clone)]
pub(crate) struct TextTransform {
    /// The position of the [`Text`] on the [`Page`].
    ///
    /// [`Page`]: crate::types::hierarchy::page::Page
    position: Position,

    /// The font size of the [`Text`] in user space units.
    size: u32,
}

/// A PDF text object, encapsulating a selected font, size, position, and content for rendering
/// text on a [`Page`].
///
/// [`Page`]: crate::types::hierarchy::page::Page
#[derive(Debug, Clone)]
pub struct Text {
    /// Represents the content (literal) to be rendered.
    content: PdfString,

    /// Represents the [`Text`] objects rendering position and scale.
    transform: TextTransform,

    /// Represents the color information used to render the given text.
    color: Color,
}

impl Text {
    /// Represents the Begin Text marker.
    pub const BT_MARKER: &[u8] = b"BT";
    /// Represents the End Text marker.
    pub const ET_MARKER: &[u8] = b"ET";

    /// Represents the Tf (Text Font) operator.
    pub const TF_OPERATOR: &[u8] = b"Tf";
    /// Represents the Td (Text Move) operator.
    pub const TD_OPERATOR: &[u8] = b"Td";
    /// Represents the Tj (Text Show) operator.
    pub const TJ_OPERATOR: &[u8] = b"Tj";

    /// Creates a default initialized [`TexBuilder`], providing default values for font (Helvetica) and it's
    /// size (12).
    pub fn builder() -> TextBuilder<false> {
        let txt = Self {
            content: PdfString::from(""),
            transform: TextTransform {
                position: Position::from_mm(0.0, 0.0),
                size: 12,
            },
            color: Color::Rgb {
                red: 0,
                green: 0,
                blue: 0,
            },
        };

        TextBuilder { inner: txt }
    }

    /// Expands the inner content with the provided one.
    fn expand(&mut self, content: impl Into<String>) {
        self.content.expand(content);
    }

    /// Returns a byte representation for drawing operations of this `Text` object in PDF syntax.
    pub(crate) fn to_bytes(&self, font_name: Identifier<&[u8]>) -> io::Result<Vec<u8>> {
        let mut writer = Vec::new();

        // BT
        writer.write_all(Self::BT_MARKER)?;
        writer.write_all(constants::NL_MARKER)?;

        self.color.write_non_stroke(&mut writer)?;

        // /FName Size Tf
        font_name.write(&mut writer)?;
        writer.write_all(format! {"{} ", self.transform.size}.as_bytes())?;
        writer.write_all(Self::TF_OPERATOR)?;
        writer.write_all(constants::NL_MARKER)?;

        // posx posy Td
        writer.write_all(
            format!(
                "{} {} ",
                self.transform.position.x, self.transform.position.y
            )
            .as_bytes(),
        )?;
        writer.write_all(Self::TD_OPERATOR)?;
        writer.write_all(constants::NL_MARKER)?;

        // (Text) Tj
        self.content.write_content(&mut writer)?;
        writer.write_all(constants::SP)?;
        writer.write_all(Self::TJ_OPERATOR)?;
        writer.write_all(constants::NL_MARKER)?;

        // ET
        writer.write_all(Self::ET_MARKER)?;
        writer.write_all(constants::NL_MARKER)?;

        Ok(writer)
    }
}

/// A builder for constructing a [`Text`] object, allowing incremental modifications.
/// The `IS_INIT` const generic tracks whether initialization has been completed (if position has
/// been set).
#[derive(Debug, Clone)]
pub struct TextBuilder<const IS_INIT: bool> {
    /// The underlying [`Text`] object being built.
    inner: Text,
}

impl<const IS_INIT: bool> TextBuilder<IS_INIT> {
    /// Sets the position of the [`Text`] on a page, after which the building of the [`Text`]
    /// object is allowed.
    pub fn at(mut self, pos: Position) -> TextBuilder<true> {
        self.inner.transform.position = pos;
        TextBuilder { inner: self.inner }
    }

    /// Sets the content of the [`Text`].
    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.inner.content = PdfString::from(content);
        self
    }

    /// Expands the content of the [`Text`] with the provided content.
    pub fn with_expanded_content(mut self, content: impl Into<String>) -> Self {
        self.inner.expand(content);
        self
    }

    /// Sets the size of the [`Text`].
    pub fn with_size(mut self, size: u32) -> Self {
        self.inner.transform.size = size;
        self
    }

    /// Sets the color of the [`Text`].
    pub fn with_color(mut self, color: Color) -> Self {
        self.inner.color = color;
        self
    }
}

impl TextBuilder<true> {
    /// Creates the [`Text`] object from the already provided configurations.
    pub fn build(self) -> Text {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use crate::types::hierarchy::{content::text::Identifier, primitives::rectangle::Position};

    use super::Text;

    #[test]
    pub fn default_text() {
        let txt = Text::builder()
            .at(Position::from_mm(0.0, 0.0))
            .build()
            .to_bytes(Identifier::from_static(b"BiHDef"))
            .unwrap();

        let output = String::from_utf8_lossy(&txt);
        insta::assert_snapshot!(output, @r"
        BT
        /DeviceRGB cs
        0 0 0 sc
        /BiHDef 12 Tf
        0 0 Td
        () Tj
        ET
        ");
    }

    #[test]
    pub fn custom_text() {
        let txt = Text::builder()
            .with_content("This is")
            .with_expanded_content(" a custom text content.")
            .with_size(14)
            .at(Position::from_mm(0.0, 0.0))
            .build()
            .to_bytes(Identifier::from_static(b"CustomFnt"))
            .unwrap();

        let output = String::from_utf8_lossy(&txt);
        insta::assert_snapshot!(output, @r"
        BT
        /DeviceRGB cs
        0 0 0 sc
        /CustomFnt 14 Tf
        0 0 Td
        (This is a custom text content.) Tj
        ET
        ");
    }
}
