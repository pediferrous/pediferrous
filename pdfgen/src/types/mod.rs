pub mod hierarchy;
pub mod pdf_writer;
pub use hierarchy::page;

/// Common constants used when writing encoded PDF into a [`Write`] or [`PdfWriter`].
///
/// [`Write`]: std::io::Write
/// [`PdfWriter`]: super::pdf_writer::PdfWriter
pub(crate) mod constants {
    /// New line constant.
    pub const NL_MARKER: &[u8] = b"\n";

    /// Single Space.
    pub const SP: &[u8] = b" ";

    /// Marker indicating end of an object section.
    pub const END_OBJ_MARKER: &[u8] = b"endobj";

    /// Default font name.
    pub const DEFAULT_FONT: &[u8] = b"BiHDef";
}
