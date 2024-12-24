//! Comment

use crate::types;

use super::{name::Name, object::Object};

/// Comment
pub struct Font<const N: usize> {
    /// Comment
    subtype: &'static [u8; N],
    /// Comment
    base_type: &'static [u8; N],
}

impl<const N: usize> Font<N> {
    const FONT: Name = Name::new(b"Font");
    const SUBTYPE: Name = Name::new(b"Subtype");
    const BASE_FONT: Name = Name::new(b"BaseFont");
}
