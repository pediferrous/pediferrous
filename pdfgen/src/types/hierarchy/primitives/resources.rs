//! Implementation of Resources Dictionary data type.

use std::io::{Error, Write};

use crate::{ObjId, types::hierarchy::content::image::Image};

use super::{
    font::Font,
    identifier::{Identifier, OwnedIdentifier},
};

/// Represents a single entry in the [`Resources`] dictionary.
#[derive(Debug)]
#[non_exhaustive]
pub(crate) enum ResourceEntry {
    Image {
        name: OwnedIdentifier,
        id: ObjId<Image>,
    },
    Font {
        name: OwnedIdentifier,
        id: ObjId<Font>,
    },
}

/// Resource dictionary enumerates the named resources needed by the operators in the content
/// stream and the names by which they can be referred to.
///
/// This is used in cases, where an operator shall refer to a PDF object that is defined outside
/// the content stream, such as a font dictionary or a stream containing image data. This shall be
/// accomplished by defining such objects as named resources and referring to them by name from
/// within the content stream.
#[derive(Default, Debug)]
pub struct Resources {
    counter: usize,
    entries: Vec<ResourceEntry>,
}

impl Resources {
    /// Creates a new [`OwnedIdentifier`] with a given prefix and internally maintained index.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut res = Resources::default();
    /// let name = res.create_name("Im");
    /// assert_eq!(name.as_bytes(), b"/Im1 ");
    /// ```
    fn create_name(&mut self, prefix: &str) -> Identifier<Vec<u8>> {
        self.counter += 1;
        Identifier::new(format!("{prefix}{}", self.counter).into_bytes())
    }

    /// Adds a reference to an [`Image`] to this `Resources` dictionary.
    ///
    /// [`Image`]: crate::types::hierarchy::content::image::Image
    pub(crate) fn add_image(&mut self, image_id: ObjId<Image>) -> Identifier<&[u8]> {
        let name = self.create_name("Im");
        let img = ResourceEntry::Image { name, id: image_id };

        self.entries.push(img);

        let ResourceEntry::Image { name, .. } = self.entries.last().unwrap() else {
            unreachable!("We added an image.")
        };

        name.as_ref()
    }

    /// Adds a reference to a [`Font`] to this `Resources` dictionary.
    ///
    /// [`Font`]: crate::types::hierarchy::primitives::font::Font
    pub(crate) fn add_font(&mut self, font_id: ObjId<Font>) -> Identifier<&[u8]> {
        let name = self.create_name("F");
        let fnt = ResourceEntry::Font { name, id: font_id };

        self.entries.push(fnt);

        let ResourceEntry::Font { name, .. } = self.entries.last().unwrap() else {
            unreachable!("We added a font.")
        };

        name.as_ref()
    }

    /// Encode and write this resource dictionary into the provided implementor of [`Write`].
    pub(crate) fn write_dict(&self, writer: &mut dyn Write) -> Result<usize, Error> {
        let mut written = writer.write(b"<< ")?;

        let mut images = self
            .entries
            .iter()
            .filter_map(|entry| match entry {
                ResourceEntry::Image { name, id } => Some((name, id)),
                ResourceEntry::Font { .. } => None,
            })
            .peekable();

        if images.peek().is_some() {
            written += Identifier::X_OBJECT.write(writer)?;
            written += writer.write(b"<< ")?;

            for (name, id) in &mut images {
                written += name.write(writer)?;
                written += id.write_ref(writer)?;
                written += writer.write(b" ")?;
            }

            written += writer.write(b">> ")?;
        }

        let mut fonts = self
            .entries
            .iter()
            .filter_map(|entry| match entry {
                ResourceEntry::Font { name, id } => Some((name, id)),
                ResourceEntry::Image { .. } => None,
            })
            .peekable();

        if fonts.peek().is_some() {
            written += Identifier::FONT.write(writer)?;
            written += writer.write(b"<< ")?;

            for (name, id) in &mut fonts {
                written += name.write(writer)?;
                written += id.write_ref(writer)?;
                written += writer.write(b" ")?;
            }

            written += writer.write(b">>")?;
        }

        written += writer.write(b" >>")?;

        Ok(written)
    }
}
