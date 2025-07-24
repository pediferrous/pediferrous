use std::io::Write;

use crate::types::{constants, hierarchy::primitives::identifier::Identifier};

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
        cyan: u8,
        /// Magenta component of the color space in range [0, 255].
        magenta: u8,
        /// Yellow component of the color space in range [0, 255].
        yellow: u8,
        /// Black component of the color space in range [0, 255].
        black: u8,
    },
}

struct ValuesIter {
    /// Holds the values of color definition. We have at most 4 values for CMYK color space, and at
    /// least 1 value for Gray color space. Unavailable values are marked with [`None`].
    values: [Option<u8>; 4],
    idx: usize,
}

impl From<Color> for ValuesIter {
    fn from(color: Color) -> Self {
        let values = match color {
            Color::Rgb { red, green, blue } => [Some(red), Some(green), Some(blue), None],
            Color::Gray(gray) => [Some(gray), None, None, None],
            Color::CMYK {
                cyan,
                magenta,
                yellow,
                black,
            } => [Some(cyan), Some(magenta), Some(yellow), Some(black)],
        };

        Self { values, idx: 0 }
    }
}

impl Iterator for ValuesIter {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.values.get(self.idx)?;
        self.idx += 1;

        match value {
            Some(value) => {
                let value = f32::from(*value) / f32::from(u8::MAX);
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

    fn identifier(&self) -> Identifier<&'static [u8]> {
        match self {
            Color::Rgb { .. } => Identifier::from_static(b"DeviceRGB"),
            Color::Gray(_) => Identifier::from_static(b"DeviceGray"),
            Color::CMYK { .. } => Identifier::from_static(b"DeviceCMYK"),
        }
    }

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
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn device_rgb() {
        let mut writer = Vec::new();
        let color = Color::Rgb {
            red: 255,
            green: 128,
            blue: 55,
        };

        color.write_stroke(&mut writer).unwrap();
        color.write_non_stroke(&mut writer).unwrap();

        let output = String::from_utf8(writer).unwrap();
        insta::assert_snapshot!(output, @r"
        /DeviceRGB CS
        1 0.5019608 0.21568628 SC
        /DeviceRGB cs
        1 0.5019608 0.21568628 sc
        ");
    }
}
