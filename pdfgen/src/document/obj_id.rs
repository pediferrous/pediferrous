//! Implementation of PDF object reference.

use std::{
    io::{Error, Write},
    marker::PhantomData,
};

/// Any object in a PDF file may be labelled as an indirect object. This gives the object a unique
/// object identifier by which other objects can refer to it. The object may be referred to from
/// elsewhere in the file by an indirect reference. Such indirect references shall consist of the
/// object number, the generation number, and the keyword R (with whitespace separating each part).
///
/// Example: `4 0 R`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObjId<T = ()> {
    /// Identifier of referenced object.
    id: u64,

    /// Marks the type of object this ObjId refers to.
    _marker: PhantomData<T>,
}

impl<T> Clone for ObjId<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            _marker: PhantomData,
        }
    }
}

impl<T> ObjId<T> {
    /// Marker indicating start of an object section
    const START_OBJ_MARKER: &[u8] = b"obj";

    /// Write the encoded PDF object reference into the provided implementor of [`Write`].
    pub fn write_ref(&self, writer: &mut dyn Write) -> Result<usize, Error> {
        Ok(pdfgen_macros::write_chain! {
            crate::write_fmt!(&mut *writer, "{}", self.id),
            // NOTE: generation is always 0 because we are genereting new PDFs and don't support
            //       updating existing PDFs
            writer.write(b" 0 R"),
        })
    }

    /// Write the encoded PDF object id into the provided implementor of [`Write`].
    pub fn write_def(&self, writer: &mut dyn Write) -> Result<usize, Error> {
        Ok(pdfgen_macros::write_chain! {
            crate::write_fmt!(&mut *writer, "{}", self.id),
            // NOTE: generation is always 0 because we are genereting new PDFs and don't support
            //       updating existing PDFs
            writer.write(b" 0 "),
            writer.write(Self::START_OBJ_MARKER),
        })
    }

    pub(crate) fn cast<U>(self) -> ObjId<U> {
        ObjId {
            id: self.id,
            _marker: PhantomData,
        }
    }
}

pub(crate) struct IdManager {
    curr: u64,
}

impl IdManager {
    pub(crate) fn new() -> Self {
        Self { curr: 1 }
    }

    /// Creates a clone of this [`IdManager`]. Take great care when doing this, otherwise the
    /// document might get into an invalid state.
    pub(in crate::document) fn clone(&self) -> Self {
        Self { curr: self.curr }
    }

    pub fn create_id<T>(&mut self) -> ObjId<T> {
        let inner_id = self.curr;
        self.curr += 1;
        ObjId {
            id: inner_id,
            _marker: PhantomData,
        }
    }
}
