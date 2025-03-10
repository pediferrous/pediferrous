//! Implementation of PDF Text object.

use crate::types::{
    constants,
    hierarchy::primitives::{object::Object, rectangle::Position, string::PdfString},
};

/// Comment
#[derive(Debug)]
pub(crate) struct TextTransform {
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
    font_name: String,
}

impl Text {
    /// Comment
    pub const BT_MARKER: &[u8] = b"BT";
    /// Comment
    pub const ET_MARKER: &[u8] = b"ET";
    /// Comment
    pub const TF_OPERATOR: &[u8] = b"Tf";
    /// Comment
    pub const TD_OPERATOR: &[u8] = b"Td";
    /// Comment
    pub const TJ_OPERATOR: &[u8] = b"Tj";

    /// Comment
    fn builder() -> TextBuilder<false> {
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

    /// Comment
    fn expand(&mut self, content: impl Into<String>) {
        self.content.expand(content);
    }
}

impl Object for Text {
    /// Comment
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

    /// Comment
    fn write_end(&self, writer: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        Ok(pdfgen_macros::write_chain! {
            writer.write(Self::ET_MARKER),
            writer.write(constants::NL_MARKER),
        })
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
    /// Comment
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
