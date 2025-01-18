pub mod hierarchy;
pub mod pdf_writer;
pub use hierarchy::page;

/// Common constants used when writing encoded PDF into a [`Write`] or [`PdfWriter`].
///
/// [`Write`]: std::io::Write
/// [`PdfWriter`]: super::pdf_writer::PdfWriter
pub mod constants {
    /// New line constant
    pub const NL_MARKER: &[u8] = b"\n";

    /// Single Space
    pub const SP: &[u8] = b" ";
}

/// Helper macro for counting the number of written bytes in multiple consecutive writes, where
/// each write returns a `Result<usize, std::io::Error>`
///
/// # Example
///
/// ```ignore
/// let mut writer = Vec::new();
/// let written = write_chain! {
///     writer.write(b"Hello"),
///     writer.write(b", World!"),
/// };
///
/// assert_eq!(written, 13);
/// ```
macro_rules! write_chain {
    ($($expression:expr),* $(,)?) => {{
        let mut written = 0;
        $(
            written += $expression?;
        )*
        written
    }}
}

pub(crate) use write_chain;

#[cfg(test)]
mod tests {
    #[test]
    fn write_chain() {
        use std::io::Write;

        let write_fn = || -> Result<usize, std::io::Error> {
            let mut writer = Vec::new();
            let written = write_chain! {
                writer.write(b"Hello"),
                writer.write(b", World!"),
            };

            Ok(written)
        };

        let written = write_fn().unwrap();
        assert_eq!(written, 13);
    }
}
