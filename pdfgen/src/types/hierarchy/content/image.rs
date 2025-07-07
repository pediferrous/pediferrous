//! Image PDF object types and implementations.

use std::io::{BufReader, Cursor, Error, Read, Write};

use image::ImageReader;
use pdfgen_macros::const_names;

use crate::{
    ObjId,
    types::{
        constants,
        hierarchy::primitives::{name::Name, object::Object, rectangle::Position, unit::Unit},
    },
};

use super::stream::Stream;

/// The colour space in which image samples shall be specified; it can be any type of colour space
/// except Pattern.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[allow(dead_code)]
enum ColorSpace {
    /// Device default RGB representation.
    DeviceRgb,

    /// Device default single gray channel representation.
    DeviceGray,
}

impl ColorSpace {
    /// Encode this `ColorSpace` into the given implementor of [`Write`].
    fn write(&self, writer: &mut dyn Write) -> Result<usize, Error> {
        match self {
            ColorSpace::DeviceRgb => Name::new(b"DeviceRGB").write(writer),
            ColorSpace::DeviceGray => Name::new(b"DeviceGray").write(writer),
        }
    }
}

/// Represents the information that should be encoded in the dictionary of an [`Image`] stream.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct ImageDict {
    /// The width of the image, in samples.
    width: u32,

    /// The height of the image, in samples.
    height: u32,

    /// The colour space in which image samples shall be specified.
    color_space: ColorSpace,

    /// The number of bits used to represent each colour component. Only a single value shall be
    /// specified; the number of bits shall be the same for all colour components. The value shall
    /// be 1, 2, 4, 8, or (from PDF 1.5) 16. If ImageMask is true, this entry is optional, but if
    /// specified, its value shall be 1.
    bits_per_comp: u8,
}

/// Represents transformations that should be applied to the encoded [`Image`] such as position and
/// scaling.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ImageTransform {
    /// Represents the position of an [`Image`] on the [`Page`].
    ///
    /// [`Page`]: crate::types::hierarchy::page::Page
    pub position: Position,
    /// Represents the scaling of an [`Image`] on the [`Page`].
    ///
    /// [`Page`]: crate::types::hierarchy::page::Page
    pub scale: Position,
}

/// A sampled image (or just image for short) is a rectangular array of sample values, each
/// representing a colour.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Image {
    /// Raw bytes of the image containing the samples. For an RGB image, each sample is represented
    /// by three values, one for each color component - red, green and blue.
    // NOTE: The source format for an image shall be described by four parameters:
    //       • The width of the image in samples
    //       • The height of the image in samples
    //       • The number of colour components per sample
    //       • The number of bits per colour component
    samples: Stream,

    /// Contains the dictionary for the image encoding the specific information of this image such
    /// as width and height in number of samples.
    dict: ImageDict,

    /// Transformations that should be applied to this image when encoding it into a
    /// [`ContentStream`].
    ///
    /// [`ContentStream`]: super::ContentStream
    transform: ImageTransform,
}

impl Image {
    const_names! {
        SUBTYPE,
        IMAGE,
        WIDTH,
        HEIGHT,
        COLOR_SPACE,
        BITS_PER_COMPONENT,
    }

    /// Creates a new [`Image`] by reading the bytes from the `reader` with default width and
    /// height of 100 mm and position 0, 0 (lower left corner of a page).
    pub fn from_reader(reader: impl Read) -> ImageBuilder<false> {
        let mut bytes = Vec::new();
        BufReader::new(reader).read_to_end(&mut bytes).unwrap();
        Self::from_bytes(bytes)
    }

    pub fn from_file(file: &std::fs::File) -> ImageBuilder<false> {
        let mut bytes = Vec::new();
        BufReader::new(file).read_to_end(&mut bytes).unwrap();
        Self::from_bytes(bytes)
    }

    /// Creates a new [`Image`] from the given bytes with default width and height of 100 mm and
    /// position 0, 0 (lower left corner of a page).
    pub fn from_bytes(bytes: impl Into<Vec<u8>>) -> ImageBuilder<false> {
        let bufreader = Cursor::new(bytes.into());

        let decoded_image = ImageReader::new(bufreader)
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();

        let img = decoded_image.to_rgb8();
        let (width, height) = img.dimensions();
        let pixels = img.into_raw();

        let img = Self {
            samples: Stream::with_bytes(pixels),
            dict: ImageDict {
                width,
                height,
                color_space: ColorSpace::DeviceRgb,
                bits_per_comp: 8,
            },
            transform: ImageTransform {
                position: Position::from_mm(0.0, 0.0),
                scale: Position::from_units(width as f32, height as f32),
            },
        };

        ImageBuilder { inner: img }
    }

    /// Sets the width and height of this [`Image`].
    pub fn set_dimensions(&mut self, width: Unit, height: Unit) {
        self.set_width(width);
        self.set_height(height);
    }

    /// Sets the width of this [`Image`]
    pub fn set_width(&mut self, width: Unit) {
        self.transform.scale.x = width;
    }

    /// Sets the height of this [`Image`]
    pub fn set_height(&mut self, height: Unit) {
        self.transform.scale.y = height;
    }

    /// Sets the position of this [`Image`].
    pub fn set_pos(&mut self, position: Position) {
        self.transform.position = position;
    }

    /// Returns the width, height and position tuple of this [`Image`].
    // TODO: should this be exposed in public API?
    pub fn transform(&self) -> ImageTransform {
        self.transform
    }

    pub fn write(&self, writer: &mut dyn Write, id: &ObjId) -> Result<usize, Error> {
        Ok(pdfgen_macros::write_chain! {
            id.write_def(writer),
            writer.write(constants::NL_MARKER),

            self.write_content(writer),
            self.write_end(writer),
        })
    }
}

impl Object for Image {
    fn write_def(&self, _writer: &mut dyn Write) -> Result<usize, Error> {
        panic!("Image does not fully implement the Object trait.")
    }

    fn write_content(&self, writer: &mut dyn Write) -> Result<usize, Error> {
        // NOTE: The image dictionary shall specify the width, height, and number of bits per
        //       component explicitly. The number of colour components shall be inferred from the
        //       colour space specified in the dictionary.

        Ok(pdfgen_macros::write_chain! {
            self.samples.write_with_dict(writer, |writer| {
                Ok(pdfgen_macros::write_chain! {
                    Name::TYPE.write(writer),
                    Name::X_OBJECT.write(writer),
                    writer.write(constants::NL_MARKER),

                    Self::SUBTYPE.write(writer),
                    Self::IMAGE.write(writer),
                    writer.write(constants::NL_MARKER),

                    Self::WIDTH.write(writer),
                    writer.write(format!("{}", self.dict.width).as_bytes()),
                    writer.write(constants::NL_MARKER),

                    Self::HEIGHT.write(writer),
                    writer.write(format!("{}", self.dict.height).as_bytes()),
                    writer.write(constants::NL_MARKER),

                    Self::COLOR_SPACE.write(writer),
                    self.dict.color_space.write(writer),
                    writer.write(constants::NL_MARKER),

                    Self::BITS_PER_COMPONENT.write(writer),
                    writer.write(format!("{}", self.dict.bits_per_comp).as_bytes()),
                    writer.write(constants::NL_MARKER),
                })
            }),
            writer.write(constants::NL_MARKER),
        })
    }
}

pub struct ImageBuilder<const IS_INIT: bool> {
    inner: Image,
}

impl<const IS_INIT: bool> ImageBuilder<IS_INIT> {
    /// Sets the position of an [`Image`] on a page.
    pub fn at(mut self, pos: Position) -> ImageBuilder<true> {
        self.inner.transform.position = pos;
        ImageBuilder { inner: self.inner }
    }

    /// Sets the scaling of the image to the given width and height.
    pub fn scaled(mut self, scale: Position) -> Self {
        self.inner.transform.scale = scale;
        self
    }

    /// This is not yet implemented and is a no-op for now.
    pub fn rotated(self, _degree: usize) -> Self {
        // TODO: implement rotation
        self
    }
}

impl ImageBuilder<true> {
    pub fn build(self) -> Image {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{IdManager, types::hierarchy::primitives::rectangle::Position};

    use super::Image;

    #[test]
    fn sample_image() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sample_image.jpg");
        let img_file = std::fs::File::open(dbg!(path)).unwrap();

        let mut id_mngr = IdManager::new();

        let img = Image::from_reader(img_file)
            .scaled(Position::from_mm(100., 100.))
            .at(Position::from_mm(10.0, 42.0))
            .build();

        let mut writer = Vec::default();
        // NOTE: same function defined on Image directly, so call using qualified syntax
        img.write(&mut writer, &id_mngr.create_id()).unwrap();
        let output = String::from_utf8_lossy(&writer);

        insta::assert_snapshot!(output);
    }
}
