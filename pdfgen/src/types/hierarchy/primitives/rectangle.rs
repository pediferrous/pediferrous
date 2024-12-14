use std::io::{Error, Write};

use super::unit::Unit;

/// Represents a point (pair of x and y coordinates) in default user space units.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Position {
    /// X coordinate in user space unit.
    x: Unit,

    /// y coordinate in user space unit.
    y: Unit,
}

impl Position {
    pub const fn new(x: Unit, y: Unit) -> Self {
        Self { x, y }
    }

    pub const fn from_mm(x: f32, y: f32) -> Self {
        Self {
            x: Unit::from_mm(x),
            y: Unit::from_mm(y),
        }
    }

    pub const fn from_units(x: f32, y: f32) -> Position {
        Self {
            x: Unit::from_unit(x),
            y: Unit::from_unit(y),
        }
    }
}

/// Rectangles are used to describe locations on a page and bounding boxes for a variety of
/// objects. A rectangle shall be written as an array of four numbers giving the coordinates of a
/// pair of diagonally opposite corners.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rectangle {
    /// Lower left corner of the `Rectangle`.
    low_left: Position,

    /// Upper right corner of the `Rectangle`.
    top_right: Position,
}

macro_rules! gen_page_constants {
    ($($name:ident, $width:literal, $height:literal),* $(,)?) => {
        $(
        pub const $name: Self = Self::new(Position::from_mm(0.0, 0.0), Position::from_mm($width - 1.0, $height - 1.0));
        )*
    }
}

impl Rectangle {
    gen_page_constants! {
        A0, 841.0, 1189.0,
        A1, 594.0, 841.0,
        A2, 420.0, 594.0,
        A3, 297.0, 420.0,
        A4, 210.0, 297.0,
        A5, 595.0, 842.0,
        A6, 595.0, 842.0,
        A7, 595.0, 842.0,
        A8, 595.0, 842.0,
        A9, 595.0, 842.0,
    }

    /// Create a new [`Rectangle`] with given [`Position`]s as its corners.
    pub const fn new(low_left: Position, top_right: Position) -> Self {
        Self {
            low_left,
            top_right,
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

    pub fn from_units(ll_x: f32, ll_y: f32, tr_x: f32, tr_y: f32) -> Self {
        Self {
            low_left: Position::from_units(ll_x, ll_y),
            top_right: Position::from_units(tr_x, tr_y),
        }
    }
}

impl From<(u32, u32, u32, u32)> for Rectangle {
    fn from((ll_x, ll_y, tr_x, tr_y): (u32, u32, u32, u32)) -> Self {
        Self::from((ll_x as f32, ll_y as f32, tr_x as f32, tr_y as f32))
    }
}

impl From<(f32, f32, f32, f32)> for Rectangle {
    fn from((ll_x, ll_y, tr_x, tr_y): (f32, f32, f32, f32)) -> Self {
        Self {
            low_left: Position::from_mm(ll_x, ll_y),
            top_right: Position::from_mm(tr_x, tr_y),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::hierarchy::primitives::unit::Unit;

    use super::Rectangle;

    #[test]
    fn new_rectangle() {
        let rect = Rectangle::from((24, 25, 42, 43));

        assert_eq!(rect.low_left.x, Unit::from_mm(24.0));
        assert_eq!(rect.low_left.y, Unit::from_mm(25.0));
        assert_eq!(rect.top_right.x, Unit::from_mm(42.0));
        assert_eq!(rect.top_right.y, Unit::from_mm(43.0));
    }

    #[test]
    fn output() {
        let rect = Rectangle::from_units(24.0, 25.0, 42.0, 43.0);

        let mut output = Vec::new();
        rect.write(&mut output).unwrap();
        let output = String::from_utf8(output).unwrap();

        insta::assert_snapshot!(output, @"[24 25 42 43]");
    }
}
