use std::io::Write;

use crate::types::{constants, hierarchy::primitives::identifier::Identifier};

mod cmyk_value;
pub use cmyk_value::CmykValue;

/// A PDF file may specify abstract colours in a device-independent way. Colours may be described
/// in any of a variety of colour systems, or colour spaces. Some colour spaces are related to
/// device colour representation (grayscale, RGB, CMYK), others to human visual perception
/// (CIE-based). Certain special features are also modelled as colour spaces: patterns, colour
/// mapping, separations, and high-fidelity and multitone colour.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// Colours in the DeviceRGB colour space shall be specified according to the additive RGB
    /// (red-green- blue) colour model, in which colour values shall be defined by three components
    /// representing the intensities of the additive primary colourants red, green, and blue. Each
    /// component shall be specified by a number in the range 0 to 255, where 0 shall denote the
    /// complete absence of a primary component and 255 shall denote maximum intensity.
    ///
    /// Note that these values will be mapped to range [0.0, 1.0] when encoding in PDF file.
    Rgb {
        /// Red component of the color space in range [0, 255].
        red: u8,
        /// Green component of the color space in range [0, 255].
        green: u8,
        /// Blue component of the color space in range [0, 255].
        blue: u8,
    },

    /// Black, white, and intermediate shades of gray are special cases of full colour. A grayscale
    /// value shall be represented by a single number in the range 0 to 255, where 0 corresponds to
    /// black, 255 to white, and intermediate values to different gray levels.
    ///
    /// Note that these values will be mapped to range [0.0, 1.0] when encoding in PDF file.
    Gray(u8),

    /// The DeviceCMYK colour space allows colours to be specified according to the subtractive
    /// CMYK (cyan-magenta-yellow-black) model typical of printers and other paper-based output
    /// devices. The four components in a DeviceCMYK colour value shall represent the
    /// concentrations of these process colourants. Each component shall be a number in the range
    /// 0 to 255, where 0 shall denote the complete absence of a process colourant and 255 shall
    /// denote maximum concentration (absorbs as much as possible of the additive primary).
    ///
    /// Note that these values will be mapped to range [0.0, 1.0] when encoding in PDF file.
    CMYK {
        /// Cyan component of the color space in range [0, 255].
        cyan: CmykValue,
        /// Magenta component of the color space in range [0, 255].
        magenta: CmykValue,
        /// Yellow component of the color space in range [0, 255].
        yellow: CmykValue,
        /// Black component of the color space in range [0, 255].
        black: CmykValue,
    },
}

struct ValuesIter {
    /// Holds the values of color definition. We have at most 4 values for CMYK color space, and at
    /// least 1 value for Gray color space. Unavailable values are marked with [`None`].
    values: [Option<u8>; 4],
    idx: usize,
    max_value: u8,
}

impl From<Color> for ValuesIter {
    fn from(color: Color) -> Self {
        match color {
            Color::Rgb { red, green, blue } => Self {
                values: [Some(red), Some(green), Some(blue), None],
                idx: 0,
                max_value: 255,
            },
            Color::Gray(gray) => Self {
                values: [Some(gray), None, None, None],
                idx: 0,
                max_value: 255,
            },
            Color::CMYK {
                cyan,
                magenta,
                yellow,
                black,
            } => Self {
                values: [
                    Some(cyan.into()),
                    Some(magenta.into()),
                    Some(yellow.into()),
                    Some(black.into()),
                ],
                idx: 0,
                max_value: 100,
            },
        }
    }
}

impl Iterator for ValuesIter {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.values.get(self.idx)?;
        self.idx += 1;

        match value {
            Some(value) => {
                let value = f32::from(*value) / f32::from(self.max_value);
                Some(value)
            }
            None => None,
        }
    }
}

impl Color {
    /// Writes the color operators for stroke coloring.
    // TODO(nfejzic): remove the `allow` attribute once we start using this method.
    #[allow(dead_code)]
    pub(crate) fn write_stroke(&self, writer: &mut impl Write) -> std::io::Result<usize> {
        self.inner_write(writer, "CS", "SC", ValuesIter::from(*self))
    }

    /// Writes the color operators for non-stroke (fill) coloring.
    pub(crate) fn write_non_stroke(&self, writer: &mut impl Write) -> std::io::Result<usize> {
        self.inner_write(writer, "cs", "sc", ValuesIter::from(*self))
    }

    /// Returns the [`Identifier`] corresponding to the color space.
    fn identifier(&self) -> Identifier<&'static [u8]> {
        match self {
            Color::Rgb { .. } => Identifier::from_static(b"DeviceRGB"),
            Color::Gray(_) => Identifier::from_static(b"DeviceGray"),
            Color::CMYK { .. } => Identifier::from_static(b"DeviceCMYK"),
        }
    }

    /// Encodes and writes color space and set color operations into PDF writer.
    fn inner_write(
        &self,
        writer: &mut impl Write,
        cs_operator: &str,
        sc_operator: &str,
        values: ValuesIter,
    ) -> std::io::Result<usize> {
        Ok(pdfgen_macros::write_chain! {
            self.identifier().write(writer),
            writer.write(cs_operator.as_bytes()),
            writer.write(constants::NL_MARKER),

            for value in values.into_iter() {
                crate::write_fmt!(&mut *writer, "{value} "),
            },

            crate::write_fmt!(&mut *writer, "{sc_operator}"),
            writer.write(constants::NL_MARKER),
        })
    }

    /// Converts the current color to gray color space.
    pub fn to_gray(self) -> Self {
        match self {
            Color::Rgb { red, green, blue } => {
                // 0.299 R + 0.587 G + 0.114 B
                let gray = 0.299 * red as f32 + 0.587 * green as f32 + 0.114 * blue as f32;
                Self::Gray(gray as u8)
            }
            Color::Gray(_) | Color::CMYK { .. } => self.to_rgb().to_gray(),
        }
    }

    /// Converts the current color to RGB color space.
    pub fn to_rgb(self) -> Self {
        match self {
            Color::Rgb { .. } => self,
            Color::Gray(gray) => Self::Rgb {
                red: gray,
                green: gray,
                blue: gray,
            },
            Color::CMYK { .. } => {
                let mut iter = ValuesIter::from(self);
                let cyan = iter.next().expect("cyan is present in CMYK");
                let magenta = iter.next().expect("magenta is present in CMYK");
                let yellow = iter.next().expect("yellow is present in CMYK");
                let black = iter.next().expect("black is present in CMYK");

                let red = (1. - dbg!(cyan)) * (1. - black) * 255.;
                let green = 255. * (1. - magenta) * (1. - black);
                let blue = 255. * (1. - yellow) * (1. - black);

                dbg!(cyan * 255.);

                Self::Rgb {
                    red: dbg!(red) as u8,
                    green: dbg!(green) as u8,
                    blue: dbg!(blue) as u8,
                }
            }
        }
    }

    /// Converts the current color to CMYK color space.
    pub fn to_cmyk(self) -> Self {
        match self {
            Color::Rgb { .. } => {
                let mut iter = ValuesIter::from(self);
                // convert to [0.0, 1.0] range
                let red = iter.next().expect("red is present in RGB");
                let green = iter.next().expect("green is present in RGB");
                let blue = iter.next().expect("blue is present in RGB");

                let black = 1. - red.max(green.max(blue));
                let cyan = (1. - red - black) / (1. - black);
                let magenta = (1. - green - black) / (1. - black);
                let yellow = (1. - blue - black) / (1. - black);

                Self::CMYK {
                    cyan: CmykValue::try_from(cyan).expect("we checked correct range"),
                    magenta: CmykValue::try_from(magenta).expect("we checked correct range"),
                    yellow: CmykValue::try_from(yellow).expect("we checked correct range"),
                    black: CmykValue::try_from(black).expect("we checked correct range"),
                }
            }
            Color::Gray(_) => self.to_rgb().to_cmyk(),
            Color::CMYK { .. } => self,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::hierarchy::content::color::CmykValue;

    use super::Color;

    macro_rules! color_tests {
        ($($test_fn:ident, $color:expr, @$expected:literal ),*) => {
            $(
            #[test]
            fn $test_fn() {
                let mut writer = Vec::new();
                let color = $color;
                color.write_stroke(&mut writer).unwrap();
                color.write_non_stroke(&mut writer).unwrap();
                let output = String::from_utf8(writer).unwrap();
                insta::assert_snapshot!(output, @$expected);
            }
            )*
        };
    }

    color_tests! {
        device_rgb,
        Color::Rgb {
            red: 255,
            green: 128,
            blue: 55,
        },
        @r"
            /DeviceRGB C
            1 0.5019608 0.21568628 SC
            /DeviceRGB cs
            1 0.5019608 0.21568628 sc
            ",

        device_gray,
        Color::Gray(128),
        @r"
            /DeviceGray CS
            0.5019608 SC
            /DeviceGray cs
            0.5019608 sc
            ",

        device_cmyk,
        Color::CMYK {
            cyan: CmykValue::from_const::<50>(),
            magenta: CmykValue::from_const::<10>(),
            yellow:  CmykValue::from_const::<100>(),
            black:   CmykValue::from_const::<42>(),
        },
        @r"
    /DeviceRGB CS
    1 0.5019608 0.21568628 SC
    /DeviceRGB cs
    1 0.5019608 0.21568628 sc
    "
    }
}
