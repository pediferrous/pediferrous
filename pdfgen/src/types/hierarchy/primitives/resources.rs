//! Implementation of Resources Dictionary data type.

use std::io::{Error, Write};

use crate::{IdManager, ObjId, types::hierarchy::content::image::Image};

use super::name::{Name, OwnedName};

/// Represents a single entry in the [`Resources`] dictionary.
#[derive(Debug)]
#[non_exhaustive]
pub(crate) enum ResourceEntry {
    Image { name: OwnedName, image: Image },
    Font { name: OwnedName, id: ObjId },
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
    pub(crate) entries: Vec<ResourceEntry>,
}

impl Resources {
    /// Creates a new [`OwnedName`] with a given prefix and internally maintained index.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut res = Resources::default();
    /// let name = res.create_name("Im");
    /// assert_eq!(name.as_bytes(), b"/Im1 ");
    /// ```
    fn create_name(&mut self, prefix: &str) -> Name<Vec<u8>> {
        self.counter += 1;
        Name::new(format!("{prefix}{}", self.counter).into_bytes())
    }

    /// Adds a reference to an [`Image`] to this `Resources` dictionary.
    ///
    /// [`Image`]: crate::types::hierarchy::content::image::Image
    pub(crate) fn add_image(&mut self, image: Image) -> Name<&[u8]> {
        let name = self.create_name("Im");
        let img = ResourceEntry::Image { name, image };

        self.entries.push(img);

        let ResourceEntry::Image { name, .. } = self.entries.last().unwrap() else {
            unreachable!("We added an image.")
        };

        name.as_ref()
    }

    /// Adds a reference to a [`Font`] to this `Resources` dictionary.
    ///
    /// [`Font`]: crate::types::hierarchy::primitives::font::Font
    pub(crate) fn add_font(&mut self, font_id: ObjId) -> Name<&[u8]> {
        let name = self.create_name("F");
        let fnt = ResourceEntry::Font { name, id: font_id };

        self.entries.push(fnt);

        let ResourceEntry::Font { name, .. } = self.entries.last().unwrap() else {
            unreachable!("We added a font.")
        };

        name.as_ref()
    }

    /// Encode and write this resource dictionary into the provided implementor of [`Write`].
    pub(crate) fn write_dict(
        &self,
        writer: &mut dyn Write,
        renderables: &[Renderable],
    ) -> Result<usize, Error> {
        Ok(pdfgen_macros::write_chain! {
            writer.write(b"<< "),

            for renderable in renderables.iter() {
                renderable.write_ref(writer),
            },

            writer.write(b" >>"),
        })
    }

    pub(crate) fn renderables(&self, id_manager: &mut IdManager) -> Vec<Renderable> {
        self.entries
            .iter()
            .map(|entry| Renderable {
                // TODO: skip creating ids for Fonts (global objects).
                id: id_manager.create_id(),
                entry,
            })
            .collect()
    }
}

#[derive(Debug)]
pub(crate) struct Renderable<'entry> {
    id: ObjId,
    entry: &'entry ResourceEntry,
}

impl Renderable<'_> {
    pub(crate) fn write_def(&self, writer: &mut dyn Write) -> std::io::Result<usize> {
        match self.entry {
            ResourceEntry::Image { image, .. } => image.write(writer, &self.id),
            ResourceEntry::Font { .. } => Ok(0),
        }
    }

    pub(crate) fn write_ref(&self, writer: &mut dyn Write) -> std::io::Result<usize> {
        match self.entry {
            ResourceEntry::Image { name, .. } => Ok(pdfgen_macros::write_chain! {
                Name::X_OBJECT.write(writer),

                writer.write(b"<< "),
                name.write(writer),
                self.id
                    .write_ref(writer),
                writer.write(b" >>"),
            }),

            ResourceEntry::Font { name, id } => Ok(pdfgen_macros::write_chain! {
                Name::FONT.write(writer),

                writer.write(b"<< "),
                name.write(writer),
                id.write_ref(writer),
                writer.write(b" >>"),
            }),
        }
    }
}
