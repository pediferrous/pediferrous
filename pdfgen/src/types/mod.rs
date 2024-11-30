use std::io::{Error, Write};

pub mod hierarchy;
pub use hierarchy::page;

pub trait WriteDictValue {
    fn write(&self, writer: &mut impl Write) -> Result<usize, Error>;
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
