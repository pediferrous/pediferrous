//! Implementation of Resources Dictionary data type.

use std::io::{Error, Write};

use crate::types::{self};

use super::{
    name::{Name, OwnedName},
    obj_id::ObjId,
};

/// Represents a single entry in the [`Resources`] dictionary.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub(crate) enum ResourceEntry {
    Image { name: OwnedName, obj_ref: ObjId },
}

impl ResourceEntry {
    const X_OBJECT: Name = Name::new(b"XObject");

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
    fn create_name(&mut self, prefix: &str) -> OwnedName {
        self.counter += 1;
        OwnedName::from_bytes(format!("{prefix}{}", self.counter).into_bytes())
    }

    pub(crate) fn add_image(&mut self, obj_ref: ObjId) -> &OwnedName {
        let name = self.create_name("Im");
        let img = ResourceEntry::Image {
            name,
            obj_ref: obj_ref.clone(),
        };

        self.entries.push(img);

        let ResourceEntry::Image { name, .. } = self.entries.last().unwrap();

        name
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
