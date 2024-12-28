//! Image PDF object types and implementations.

use std::io::{Error, Read, Write};

use crate::types::{
    self, constants,
    hierarchy::primitives::{
        name::Name, obj_id::ObjId, object::Object, rectangle::Position, unit::Unit,
    },
};

use super::stream::Stream;

/// The colour space in which image samples shall be specified; it can be any type of colour space
/// except Pattern.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum ColorSpace {
    /// Device default RGB representation.
    DeviceRgb,
}

impl ColorSpace {
    fn write(&self, writer: &mut dyn Write) -> Result<usize, Error> {
        match self {
            ColorSpace::DeviceRgb => Name::new(b"DeviceRgb").write(writer),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct ImageDict {
    /// The width of the image, in samples.
    width: usize,

    /// The height of the image, in samples.
    height: usize,

    /// The colour space in which image samples shall be specified.
    color_space: ColorSpace,

    /// The number of bits used to represent each colour component. Only a single value shall be
    /// specified; the number of bits shall be the same for all colour components. The value shall
    /// be 1, 2, 4, 8, or (from PDF 1.5) 16. If ImageMask is true, this entry is optional, but if
    /// specified, its value shall be 1.
    bits_per_comp: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub(crate) struct ImageTransform {
    position: Position,
    scale: Position,
}

/// A sampled image (or just image for short) is a rectangular array of sample values, each
/// representing a colour.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Image {
    // NOTE: The source format for an image shall be described by four parameters:
    //       • The width of the image in samples
    //       • The height of the image in samples
    //       • The number of colour components per sample
    //       • The number of bits per colour component
    samples: Stream,

    dict: ImageDict,

    transform: ImageTransform,
}

impl Image {
    const TYPE: Name = Name::new(b"Type");
    const SUB_TYPE: Name = Name::new(b"Subtype");
    const IMG_SUB_TYPE: Name = Name::new(b"Image");

    const WIDTH: Name = Name::new(b"Width");
    const HEIGHT: Name = Name::new(b"Height");
    const COLOR_SPACE: Name = Name::new(b"ColorSpace");
    const BITS_PER_COMP: Name = Name::new(b"BitsPerComponent");

    /// Creates a new [`Image`] by reading the bytes from the `reader` with default width and
    /// height of 100 mm and position 0, 0 (lower left corner of a page).
    pub fn from_reader(id: ObjId, mut reader: impl Read) -> Self {
        let mut buf = Vec::new();
        let read_bytes = reader.read_to_end(&mut buf).unwrap();

        debug_assert!(read_bytes == buf.len());

        Self {
            samples: Stream::with_bytes(id, buf),
            dict: ImageDict {
                color_space: ColorSpace::DeviceRgb,
                bits_per_comp: 16,
                width: 460,
                height: 460,
            },
            transform: ImageTransform {
                position: Position::from_mm(0.0, 0.0),
                scale: Position::from_mm(0.0, 0.0),
            },
        }
    }

    /// Creates a new [`Image`] from the given bytes with default width and height of 100 mm and
    /// position 0, 0 (lower left corner of a page).
    pub fn from_bytes(id: ObjId, bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            samples: Stream::with_bytes(id, bytes),
            dict: ImageDict {
                width: 460,
                height: 460,
                color_space: ColorSpace::DeviceRgb,
                bits_per_comp: 16,
            },
            transform: ImageTransform {
                position: Position::from_mm(0.0, 0.0),
                scale: Position::from_units(1.0, 1.0),
            },
        }
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
    #[allow(dead_code)]
    pub(crate) fn transform(&self) -> ImageTransform {
        self.transform
    }

    pub fn obj_ref(&self) -> &ObjId {
        &self.samples.id
    }
}

impl Object for Image {
    fn write(&self, writer: &mut dyn Write) -> Result<usize, Error> {
        // NOTE: The image dictionary shall specify the width, height, and number of bits per
        //       component explicitly. The number of colour components shall be inferred from the
        //       colour space specified in the dictionary.

        self.samples.write_with_dict(writer, |writer| {
            Ok(types::write_chain! {
                Self::TYPE.write(writer),
                writer.write(b"XObject"),
                writer.write(constants::NL_MARKER),

                Self::SUB_TYPE.write(writer),
                Self::IMG_SUB_TYPE.write(writer),
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

                Self::BITS_PER_COMP.write(writer),
                writer.write(format!("{}", self.dict.bits_per_comp).as_bytes()),
                writer.write(constants::NL_MARKER),
            })
        })
    }

    fn obj_ref(&self) -> &ObjId {
        self.obj_ref()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::types::hierarchy::primitives::{
        obj_id::IdManager, object::Object, rectangle::Position, unit::Unit,
    };

    use super::Image;

    #[test]
    fn sample_image() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sample_image.jpg");
        let img_file = std::fs::File::open(dbg!(path)).unwrap();

        let mut id_mngr = IdManager::default();

        let mut img = Image::from_reader(id_mngr.create_id(), img_file);
        img.set_width(Unit::from_mm(100.0));
        img.set_height(Unit::from_mm(100.0));
        img.set_pos(Position::from_mm(10.0, 42.0));

        let mut writer = Vec::default();
        img.write(&mut writer).unwrap();
        let output = String::from_utf8_lossy(&writer);

        insta::assert_snapshot!(output);
    }
}
