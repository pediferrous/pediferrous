use std::io::{Error, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    x: u32,
    y: u32,
}

impl<A, B> From<(A, B)> for Position
where
    A: Into<u32>,
    B: Into<u32>,
{
    fn from((x, y): (A, B)) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rectangle {
    low_left: Position,
    top_right: Position,
}

impl Rectangle {
    pub fn new(low_left: impl Into<Position>, top_right: impl Into<Position>) -> Self {
        Self {
            low_left: low_left.into(),
            top_right: top_right.into(),
        }
    }

    pub fn write(&self, writer: &mut impl Write) -> Result<usize, Error> {
        let output = format!(
            "[{} {} {} {}]",
            self.low_left.x, self.low_left.y, self.top_right.x, self.top_right.y
        );
        let written = writer.write(output.as_bytes())?;
        Ok(written)
    }
}

#[cfg(test)]
mod tests {
    use super::Rectangle;

    #[test]
    fn new_rectangle() {
        let rect = Rectangle::new((24u8, 25u8), (42u8, 43u8));

        assert_eq!(rect.low_left.x, 24);
        assert_eq!(rect.low_left.y, 25);
        assert_eq!(rect.top_right.x, 42);
        assert_eq!(rect.top_right.y, 43);
    }

    #[test]
    fn output() {
        let rect = Rectangle::new((24u8, 25u8), (42u8, 43u8));

        let mut output = Vec::new();
        rect.write(&mut output).unwrap();
        let output = String::from_utf8(output).unwrap();

        insta::assert_snapshot!(output, @"[24 25 42 43]");
    }
}
