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

#[macro_export]
macro_rules! write_fmt {
    ($dst:expr, $($arg:tt)*) => {{
        let mut writer = $crate::macros::WriteCounter { writer: $dst, counter: 0 };

        if std::fmt::write(&mut writer, ::std::format_args!($($arg)*)).is_err() {
            return Err(std::io::Error::other("could not write formatted string"));
        }

        Ok::<usize, std::io::Error>(writer.counter)
    }};
}
