use std::{fmt, io::Write};

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
}

impl Color {
    pub fn write_stroke(&self, writer: &mut impl Write) -> std::io::Result<usize> {
        self.write(writer, "CS", "SC", self.values())
    }

    pub fn write_non_stroke(&self, writer: &mut impl Write) -> std::io::Result<usize> {
        self.write(writer, "cs", "sc", self.values())
    }

    fn values(&self) -> impl IntoIterator<Item = f32> {
        match self {
            Color::Rgb { red, green, blue } => {
                [red, green, blue].map(|val| f32::from(*val) / f32::from(u8::MAX))
            }
        }
    }

    fn identifier(&self) -> Identifier<&'static [u8]> {
        match self {
            Color::Rgb { .. } => Identifier::from_static(b"DeviceRGB"),
        }
    }

    fn write<I, T>(
        &self,
        writer: &mut impl Write,
        cs_operator: &str,
        sc_operator: &str,
        values: I,
    ) -> std::io::Result<usize>
    where
        I: IntoIterator<Item = T>,
        T: fmt::Display,
    {
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
