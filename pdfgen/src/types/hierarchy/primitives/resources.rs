//! Implementation of Resources Dictionary data type.

use std::io::{Error, Write};

use crate::types::{self};

use super::{name::Name, obj_id::ObjId};

/// Represents a single entry in the [`Resources`] dictionary.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub(crate) enum ResourceEntry {
    Image { name: Name<Vec<u8>>, obj_ref: ObjId },
}

impl ResourceEntry {
    const X_OBJECT: Name<&'static [u8]> = Name::from_static(b"XObject");

    /// Encode and write this entry into the implementor of [`Write`].
    fn write(&self, writer: &mut dyn Write) -> Result<usize, Error> {
        match self {
            ResourceEntry::Image { name, obj_ref } => Ok(types::write_chain! {
                Self::X_OBJECT.write(writer),

                writer.write(b"<< "),
                name.write(writer),
                obj_ref.write_ref(writer),
                writer.write(b" >>"),
            }),
        }
    }
}

/// Resource dictionary enumerates the named resources needed by the operators in the content
/// stream and the names by which they can be referred to.
///
/// This is used in cases, where an operator shall refer to a PDF object that is defined outside
/// the content stream, such as a font dictionary or a stream containing image data. This shall be
/// accomplished by defining such objects as named resources and referring to them by name from
/// within the content stream.
#[derive(Default, Debug, Clone)]
pub struct Resources {
    counter: usize,
    entries: Vec<ResourceEntry>,
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
    pub(crate) fn add_image(&mut self, obj_ref: ObjId) -> Name<&[u8]> {
        let name = self.create_name("Im");
        let img = ResourceEntry::Image {
            name,
            obj_ref: obj_ref.clone(),
        };

        self.entries.push(img);

        let ResourceEntry::Image { name, .. } = self.entries.last().unwrap();

        name.as_ref()
    }

    /// Encode and write this resource dictionary into the provided implementor of [`Write`].
    pub(crate) fn write(&self, writer: &mut dyn Write) -> Result<usize, Error> {
        let written = types::write_chain! {
            writer.write(b"<< "),
            self.entries.iter().map(|entry| entry.write(writer)).sum::<Result<usize, _>>(),
            writer.write(b" >>"),
        };

        Ok(written)
    }
}
