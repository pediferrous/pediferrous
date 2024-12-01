use std::io::{Error, Write};

/// Represents a point (pair of x and y coordinates) in default user space units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    /// X coordinate in user space unit.
    x: u32,

    /// y coordinate in user space unit.
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

/// Rectangles are used to describe locations on a page and bounding boxes for a variety of
/// objects. A rectangle shall be written as an array of four numbers giving the coordinates of a
/// pair of diagonally opposite corners.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rectangle {
    /// Lower left corner of the `Rectangle`.
    low_left: Position,

    /// Upper right corner of the `Rectangle`.
    top_right: Position,
}

impl Rectangle {
    /// Create a new [`Rectangle`] with given [`Position`]s as its corners.
    pub fn new(low_left: impl Into<Position>, top_right: impl Into<Position>) -> Self {
        Self {
            low_left: low_left.into(),
            top_right: top_right.into(),
        }
    }

    /// Encode and write this [`Rectangle`] into the provided implementor of [`Write`].
    pub fn write(&self, writer: &mut impl Write) -> Result<usize, Error> {
        let output = format!(
            "[{} {} {} {}]",
            self.low_left.x, self.low_left.y, self.top_right.x, self.top_right.y
        );
        let written = writer.write(output.as_bytes())?;
        Ok(written)
    }
}

impl From<(u32, u32, u32, u32)> for Rectangle {
    fn from((ll_x, ll_y, tr_x, tr_y): (u32, u32, u32, u32)) -> Self {
        Rectangle {
            low_left: (ll_x, ll_y).into(),
            top_right: (tr_x, tr_y).into(),
        }
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
