//! Image PDF object types and implementations.

use std::io::{BufReader, Cursor, Error, Read, Write};

use image::ImageReader;
use pdfgen_macros::const_identifiers;

use crate::{
    ObjId,
    types::{
        constants,
        hierarchy::primitives::{identifier::Identifier, object::Object, rectangle::Position},
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
            ColorSpace::DeviceRgb => Identifier::new(b"DeviceRGB").write(writer),
            ColorSpace::DeviceGray => Identifier::new(b"DeviceGray").write(writer),
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

/// Represents transformations that should be applied when drawing an [`Image`] on a
/// [`Page`], such as position and scaling.
///
/// [`Page`]: crate::types::hierarchy::page::Page
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

impl ImageTransform {
    /// Creates a new [`ImageTransform`] with the given position and scale.
    pub fn new(position: Position, scale: Position) -> Self {
        Self { position, scale }
    }

    /// Sets the position of the image on the page.
    pub fn at(mut self, position: Position) -> Self {
        self.position = position;
        self
    }

    /// Sets the scaling of the image to the given width and height.
    pub fn scaled(mut self, scale: Position) -> Self {
        self.scale = scale;
        self
    }

    /// This is not yet implemented and is a no-op for now.
    pub fn rotated(self, _degree: usize) -> Self {
        // TODO: implement rotation
        self
    }
}

/// A sampled image (or just image for short) is a rectangular array of sample values, each
/// representing a colour.
///
/// An `Image` holds only the image data itself. It is registered in a [`Document`] via
/// [`Document::add_image`], and placed on pages with [`Page::draw_image`], which applies an
/// [`ImageTransform`] at draw time.
///
/// [`Document`]: crate::Document
/// [`Document::add_image`]: crate::Document::add_image
/// [`Page::draw_image`]: crate::types::hierarchy::page::Page::draw_image
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
}

impl Image {
    const_identifiers! {
        SUBTYPE,
        IMAGE,
        WIDTH,
        HEIGHT,
        COLOR_SPACE,
        BITS_PER_COMPONENT,
    }

    /// Creates a new [`Image`] by reading the bytes from the `reader`.
    pub fn from_reader(reader: impl Read) -> Image {
        let mut bytes = Vec::new();
        BufReader::new(reader).read_to_end(&mut bytes).unwrap();
        Self::from_bytes(bytes)
    }

    /// Creates a new [`Image`] by reading the bytes from the given file.
    pub fn from_file(file: &std::fs::File) -> Image {
        let mut bytes = Vec::new();
        BufReader::new(file).read_to_end(&mut bytes).unwrap();
        Self::from_bytes(bytes)
    }

    /// Creates a new [`Image`] from the given bytes.
    pub fn from_bytes(bytes: impl Into<Vec<u8>>) -> Image {
        let bufreader = Cursor::new(bytes.into());

        let decoded_image = ImageReader::new(bufreader)
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();

        let img = decoded_image.to_rgb8();
        let (width, height) = img.dimensions();
        let pixels = img.into_raw();

        Self {
            samples: Stream::with_bytes(pixels),
            dict: ImageDict {
                width,
                height,
                color_space: ColorSpace::DeviceRgb,
                bits_per_comp: 8,
            },
        }
    }

    /// Encode the image stream along with its dictionary into the given implementor of [`Write`].
    pub(crate) fn write_content(&self, writer: &mut dyn Write) -> Result<usize, Error> {
        // NOTE: The image dictionary shall specify the width, height, and number of bits per
        //       component explicitly. The number of colour components shall be inferred from the
        //       colour space specified in the dictionary.

        Ok(pdfgen_macros::write_chain! {
            self.samples.write_with_dict(writer, |writer| {
                Ok(pdfgen_macros::write_chain! {
                    Identifier::TYPE.write(writer),
                    Identifier::X_OBJECT.write(writer),
                    writer.write(constants::NL_MARKER),

                    Self::SUBTYPE.write(writer),
                    Self::IMAGE.write(writer),
                    writer.write(constants::NL_MARKER),

                    Self::WIDTH.write(writer),
                    crate::write_fmt!(&mut *writer, "{}", self.dict.width),
                    writer.write(constants::NL_MARKER),

                    Self::HEIGHT.write(writer),
                    crate::write_fmt!(&mut *writer, "{}", self.dict.height),
                    writer.write(constants::NL_MARKER),

                    Self::COLOR_SPACE.write(writer),
                    self.dict.color_space.write(writer),
                    writer.write(constants::NL_MARKER),

                    Self::BITS_PER_COMPONENT.write(writer),
                    crate::write_fmt!(&mut *writer, "{}", self.dict.bits_per_comp),
                    writer.write(constants::NL_MARKER),
                })
            }),
            writer.write(constants::NL_MARKER),
        })
    }
}

/// An [`Image`] that has been registered in a [`Document`] and assigned an [`ObjId`].
///
/// [`Document`]: crate::Document
#[derive(Debug)]
pub(crate) struct RegisteredImage {
    /// ID of the registered [`Image`] object.
    id: ObjId<Image>,

    /// The registered image.
    image: Image,
}

impl RegisteredImage {
    /// Creates a new [`RegisteredImage`] from the given [`ObjId`] and [`Image`].
    pub(crate) fn new(id: ObjId<Image>, image: Image) -> Self {
        Self { id, image }
    }
}

impl Object for RegisteredImage {
    fn write_def(&self, writer: &mut dyn Write) -> Result<usize, Error> {
        Ok(pdfgen_macros::write_chain! {
            self.id.write_def(writer),
            writer.write(constants::NL_MARKER),
        })
    }

    fn write_content(&self, writer: &mut dyn Write) -> Result<usize, Error> {
        self.image.write_content(writer)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{IdManager, types::hierarchy::primitives::object::Object};

    use super::{Image, RegisteredImage};

    #[test]
    fn sample_image() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sample_image.jpg");
        let img_file = std::fs::File::open(dbg!(path)).unwrap();

        let mut id_mngr = IdManager::new();

        let img = Image::from_reader(img_file);
        let registered = RegisteredImage::new(id_mngr.create_id(), img);

        let mut writer = Vec::default();
        registered.write_def(&mut writer).unwrap();
        registered.write_content(&mut writer).unwrap();
        registered.write_end(&mut writer).unwrap();
        let output = String::from_utf8_lossy(&writer);

        insta::assert_snapshot!(output);
    }
}
