//! Implementation of PDF Text object.

use crate::types::{
    constants,
    hierarchy::primitives::{object::Object, rectangle::Position, string::PdfString},
};

/// Defines the transformation properties of a [`Text`] object, including its position and size on a [`Page`].
///
/// [`Page`]: crate::types::hierarchy::page::Page
#[derive(Debug)]
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
#[derive(Debug)]
pub struct Text {
    /// Represents the content (literal) to be rendered.
    content: PdfString,

    /// Represents the [`Text`] objects rendering position and scale.
    transform: TextTransform,

    /// Represents a PDF Resources reference name of an already defined font.
    font_name: String,
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
            font_name: String::from_utf8_lossy(constants::DEFAULT_FONT).to_string(),
        };

        TextBuilder { inner: txt }
    }

    /// Expands the inner content with the provided one.
    fn expand(&mut self, content: impl Into<String>) {
        self.content.expand(content);
    }
}

impl Object for Text {
    /// Writes the object definition part of this object, in this case `BT`, representing Begin
    /// Text.
    fn write_def(&self, writer: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        Ok(pdfgen_macros::write_chain! {
            writer.write(Self::BT_MARKER),
            writer.write(constants::NL_MARKER),
        })
    }

    fn write_content(&self, writer: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        Ok(pdfgen_macros::write_chain! {
            // /FName Size Tf
            writer.write(format!{"/{} {} ", self.font_name, self.transform.size}.as_bytes()),
            writer.write(Self::TF_OPERATOR),
            writer.write(constants::NL_MARKER),

            // posx posy Td
            writer.write(format!("{} {} ", self.transform.position.x, self.transform.position.y).as_bytes()),
            writer.write(Self::TD_OPERATOR),
            writer.write(constants::NL_MARKER),

            // (Text) Tj
            self.content.write_content(writer),
            writer.write(constants::SP),
            writer.write(Self::TJ_OPERATOR),
            writer.write(constants::NL_MARKER),
        })
    }

    /// Writes the object definition closure part of this object, in this case `ET`, representing
    /// End Text.
    fn write_end(&self, writer: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        Ok(pdfgen_macros::write_chain! {
            writer.write(Self::ET_MARKER),
            writer.write(constants::NL_MARKER),
        })
    }
}

/// A builder for constructing a [`Text`] object, allowing incremental modifications.
/// The `IS_INIT` const generic tracks whether initialization has been completed (if position has
/// been set).
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

    /// Sets the font of the [`Text`].
    pub fn with_font(mut self, font_name: impl Into<String>) -> Self {
        self.inner.font_name = font_name.into();
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
    use crate::types::hierarchy::primitives::object::Object;
    use crate::types::hierarchy::primitives::rectangle::Position;

    use super::Text;

    #[test]
    pub fn default_text() {
        let txt = Text::builder().at(Position::from_mm(0.0, 0.0)).build();

        let mut writer = Vec::default();
        let _ = txt.write_def(&mut writer);
        let _ = txt.write_content(&mut writer);
        let _ = txt.write_end(&mut writer);

        let output = String::from_utf8_lossy(&writer);
        insta::assert_snapshot!(output);
    }

    #[test]
    pub fn custom_text() {
        let txt = Text::builder()
            .with_content("This is")
            .with_expanded_content(" a custom text content.")
            .with_size(14)
            .with_font("CustomFnt")
            .at(Position::from_mm(0.0, 0.0))
            .build();

        let mut writer = Vec::default();
        let _ = txt.write_def(&mut writer);
        let _ = txt.write_content(&mut writer);
        let _ = txt.write_end(&mut writer);

        let output = String::from_utf8_lossy(&writer);
        insta::assert_snapshot!(output);
    }
}
