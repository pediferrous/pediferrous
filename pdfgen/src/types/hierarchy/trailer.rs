//! Implementation of the PDF-s trailer section.

use std::io::Write;

use crate::types::{self, constants};

use super::primitives::{name::Name, obj_ref::ObjRef};

/// Comment
pub trait WriteTrailer {
    /// Comment
    fn write(
        &self,
        writer: &mut impl Write,
        offset: usize,
        size: usize,
        root: ObjRef,
        id: [u8; 16],
    ) -> Result<usize, std::io::Error>;
}


    /// Comment
    id: u8,
}
