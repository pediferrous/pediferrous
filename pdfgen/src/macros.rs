use std::fmt;

pub(crate) struct WriteCounter<W> {
    pub(crate) writer: W,
    pub(crate) counter: usize,
}

impl<W: std::io::Write> std::fmt::Write for WriteCounter<W> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        self.writer.write_all(bytes).map_err(|_| fmt::Error)?;
        self.counter += bytes.len();
        Ok(())
    }
}

/// Helper macro for writing formatted string content into PDF writer without allocating a string.
/// Usage is very similar to [`std::write`] macro:
///
/// ```ignore
/// let mut writer = Vec::new();
/// let count = crate::write_fmt!(&mut writer, "{}", 42).unwrap();
///
/// assert_eq!(writer, b"42");
/// assert_eq!(count, 2);
/// ```
#[macro_export]
macro_rules! write_fmt {
    ($dst:expr, $($arg:tt)*) => {{
        let mut writer = $crate::macros::WriteCounter { writer: $dst, counter: 0 };

        match std::fmt::write(&mut writer, ::std::format_args!($($arg)*)) {
            Err(_) => Err(std::io::Error::other("could not write formatted string")),
            Ok(_) => Ok::<usize, std::io::Error>(writer.counter),
        }
    }}
}

#[cfg(test)]
mod tests {
    #[test]
    fn write_fmt_macro() {
        let mut writer = Vec::new();
        let count = crate::write_fmt!(&mut writer, "{}", 42).unwrap();

        assert_eq!(writer, b"42");
        assert_eq!(count, 2);
    }
}
