//! Typometry is crate for working with text metrics. In particular, calculating text metrics based
//! on provided font definitions.

use rustybuzz::{Face, UnicodeBuffer};

pub mod layout;
pub use layout::{FitResult, LayoutError, LayoutParams, Line, Size, layout_text};

pub struct RawMetrics {
    width: f64,
    line_height: f64,
}

impl RawMetrics {
    pub fn calculate(font_ttf: &[u8], text: &str) -> Vec<Self> {
        let mut face = Face::from_slice(font_ttf, 0).unwrap();
        face.set_points_per_em(Some(200.));

        let height = face.height() as i32;

        let mut result = Vec::new();
        let mut buffer = UnicodeBuffer::new();
        for line in text.lines() {
            buffer.push_str(line);

            let shaped = rustybuzz::shape(&face, &[], buffer);
            let glyph_positions = shaped.glyph_positions();

            let width = glyph_positions
                .iter()
                .fold(0, |acc, pos| acc + pos.x_advance);

            let units = face.units_per_em() as f64;

            result.push(RawMetrics {
                width: width as f64 / units,
                line_height: height as f64 / units,
            });

            buffer = shaped.clear();
        }

        result
    }

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn line_height(&self) -> f64 {
        self.line_height
    }
}

pub struct Metrics<'input> {
    /// Stores metrics of each line of text.
    metrics: Vec<RawMetrics>,

    /// Stores the content that this instance of metrics is calculated of.
    content: &'input str,
}

impl Metrics<'_> {
    pub fn calculate<'input>(font_ttf: &[u8], content: &'input str) -> Metrics<'input> {
        let metrics = RawMetrics::calculate(font_ttf, content);

        Metrics { metrics, content }
    }

    pub fn metrics(&self) -> &[RawMetrics] {
        &self.metrics
    }

    /// Returns width of each line.
    pub fn width(&self) -> impl Iterator<Item = f64> {
        self.metrics.iter().map(|raw_metrics| raw_metrics.width)
    }

    pub fn height(&self) -> f64 {
        self.metrics
            .iter()
            .map(|raw_metrics| raw_metrics.line_height)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use crate::Metrics;

    const MONOLISA_REGULAR_PATH: &str = "/Users/nfejzic/Library/Fonts/MonoLisa-Regular.ttf";

    #[test]
    fn monolisa_regular_metrics() {
        let mut font = File::open(MONOLISA_REGULAR_PATH).expect("the font exists");
        let mut font_ttf = Vec::new();
        font.read_to_end(&mut font_ttf).unwrap();

        let text = "Hello there\nhow is it going?";
        let metrics = Metrics::calculate(&font_ttf, text);

        for (line_metrics, line) in metrics.metrics().iter().zip(text.lines()) {
            println!(
                "'{line}' is {} wide and {} high",
                line_metrics.width(),
                line_metrics.line_height()
            );
        }
    }

    #[test]
    fn noto_sans_mono_regular_metrics() {
        let mut font = File::open(MONOLISA_REGULAR_PATH).expect("the font exists");
        let mut font_ttf = Vec::new();
        font.read_to_end(&mut font_ttf).unwrap();

        let text = "Hello";
        let metrics = Metrics::calculate(&font_ttf, text);

        for (line_metrics, line) in metrics.metrics().iter().zip(text.lines()) {
            println!(
                "'{line}' is {} wide and {} high",
                line_metrics.width(),
                line_metrics.line_height()
            );
        }
    }
}
