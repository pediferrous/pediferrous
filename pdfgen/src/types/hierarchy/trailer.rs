//! Implementation of the PDF-s trailer section.

use std::io::Write;

use crate::types::{self, constants};

use super::primitives::{name::Name, obj_ref::ObjRef};

/// Comment
pub struct Trailer {
    /// Comment
    size: usize,

    /// The catalog dictionary for the PDF file, representing the root of the trailer
    root: ObjRef,

    /// Comment
    id: u8,
}
