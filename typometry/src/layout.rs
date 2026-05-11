//! Minimal text layout helpers for fitting shaped text into a bounding box.
//!
//! This module provides a small API surface that can answer two common questions:
//! 1. Does all text fit into the requested rectangle?
//! 2. If not, what portion fits and what remains?

use rustybuzz::{Face, UnicodeBuffer};

/// Parameters required to lay out text into a rectangular area.
#[derive(Debug, Clone, Copy)]
pub struct LayoutParams<'font> {
    pub font_ttf: &'font [u8],
    pub font_size: f32,
    pub max_width: f32,
    pub max_height: f32,
    pub line_height: Option<f32>,
}

/// Final dimensions occupied by the laid out lines.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Metrics {
    pub width: f32,
    pub height: f32,
}

/// One rendered line candidate and its measured width.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line<'text> {
    pub text: &'text str,
    pub width: f32,
}

/// Text that fits within the requested bounds.
#[derive(Debug, Clone, PartialEq)]
pub struct ContainedText<'text> {
    pub lines: Vec<Line<'text>>,
    pub metrics: Metrics,
}

/// Result of laying out text into the provided bounds.
#[derive(Debug, Clone, PartialEq)]
pub enum TextLayout<'text> {
    Contained(ContainedText<'text>),
    Overflow {
        contained: ContainedText<'text>,
        remaining: &'text str,
    },
}

/// Errors that can occur while preparing layout inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum LayoutError {
    #[error("invalid font data")]
    InvalidFont,
    #[error("invalid font metrics")]
    InvalidFontMetrics,
    #[error("font size must be positive")]
    InvalidFontSize,
    #[error("bounds must be non-negative")]
    InvalidBounds,
    #[error("line height must be positive")]
    InvalidLineHeight,
}

/// Layout text into a rectangular box and report either complete fit or overflow.
///
/// This implementation intentionally keeps the algorithm minimal:
/// - It uses UAX #14 break opportunities (`unicode-linebreak`).
/// - It approximates candidate widths by summing per-grapheme shaped advances.
/// - It falls back to grapheme-cluster hard breaks for unbreakable runs.
///
/// Width accumulation is intentionally conservative and can underfill lines in cases where
/// contextual shaping would reduce total advance when graphemes are shaped together.
///
// TODO(nfejzic): Integrate paragraph bidi reordering with `unicode-bidi` for mixed-direction lines.
pub fn text<'text>(
    text: &'text str,
    params: &LayoutParams<'_>,
) -> Result<TextLayout<'text>, LayoutError> {
    if params.font_size <= 0.0 {
        return Err(LayoutError::InvalidFontSize);
    }

    if params.max_width < 0.0 || params.max_height < 0.0 {
        return Err(LayoutError::InvalidBounds);
    }

    if text.is_empty() {
        return Ok(TextLayout::Contained(ContainedText {
            lines: Vec::new(),
            metrics: Metrics {
                width: 0.0,
                height: 0.0,
            },
        }));
    }

    let mut face = Face::from_slice(params.font_ttf, 0).ok_or(LayoutError::InvalidFont)?;
    face.set_points_per_em(Some(params.font_size));

    let units_per_em = face.units_per_em() as f32;
    if units_per_em <= 0.0 {
        return Err(LayoutError::InvalidFontMetrics);
    }

    let default_line_height = (face.ascender() - face.descender() + face.line_gap()) as f32
        * (params.font_size / units_per_em);
    let line_height = params.line_height.unwrap_or(default_line_height);

    if line_height <= 0.0 {
        return Err(LayoutError::InvalidLineHeight);
    }

    let mut lines = Vec::new();
    let mut used_height: f32 = 0.0;
    let mut used_width: f32 = 0.0;

    let mut cursor = 0;
    while cursor <= text.len() {
        let remaining = &text[cursor..];
        let next_newline = remaining.find('\n');

        let paragraph_end = next_newline.map(|pos| cursor + pos).unwrap_or(text.len());
        let paragraph = &text[cursor..paragraph_end];

        if paragraph.is_empty() {
            if used_height + line_height > params.max_height {
                return Ok(TextLayout::Overflow {
                    contained: ContainedText {
                        lines,
                        metrics: Metrics {
                            width: used_width,
                            height: used_height,
                        },
                    },
                    remaining: &text[cursor..],
                });
            }

            // empty line, so push an empty line
            lines.push(Line {
                text: &text[cursor..cursor],
                width: 0.0,
            });
            used_height += line_height;
        } else {
            let mut local_cursor = 0;

            while local_cursor < paragraph.len() {
                let slice = &paragraph[local_cursor..];
                let (line_end, line_width) =
                    longest_fitting_prefix(&face, slice, params.font_size, params.max_width);

                let line_start = local_cursor;
                let line_end_in_paragraph = local_cursor + line_end;
                let line_text = &paragraph[line_start..line_end_in_paragraph];

                if used_height + line_height > params.max_height {
                    return Ok(TextLayout::Overflow {
                        contained: ContainedText {
                            lines,
                            metrics: Metrics {
                                width: used_width,
                                height: used_height,
                            },
                        },
                        remaining: &text[cursor + line_start..],
                    });
                }

                lines.push(Line {
                    text: line_text,
                    width: line_width,
                });

                used_height += line_height;
                used_width = used_width.max(line_width);
                local_cursor += line_end;
            }
        }

        cursor = paragraph_end + usize::from(next_newline.is_some());
    }

    Ok(TextLayout::Contained(ContainedText {
        lines,
        metrics: Metrics {
            width: used_width,
            height: used_height,
        },
    }))
}

fn longest_fitting_prefix(
    face: &Face<'_>,
    text: &str,
    font_size: f32,
    max_width: f32,
) -> (usize, f32) {
    let mut graphemes = unicode_segmentation::UnicodeSegmentation::grapheme_indices(text, true)
        .map(|(idx, grapheme)| {
            (
                idx + grapheme.len(),
                shaped_width(face, grapheme, font_size),
            )
        })
        .peekable();

    let mut breaks = unicode_linebreak::linebreaks(text)
        .map(|(idx, _)| idx)
        .filter(|idx| *idx > 0);

    let mut accumulated_width = 0.0_f32;
    let mut last_fitting_break: Option<(usize, f32)> = None;
    let mut last_fitting_grapheme: Option<(usize, f32)> = None;
    let mut first_grapheme: Option<(usize, f32)> = None;

    for break_idx in breaks.by_ref() {
        while graphemes
            .peek()
            .is_some_and(|(grapheme_end, _)| *grapheme_end <= break_idx)
        {
            let (grapheme_end, grapheme_width) =
                graphemes.next().expect("peek guarantees next item");

            accumulated_width += grapheme_width;
            if first_grapheme.is_none() {
                first_grapheme = Some((grapheme_end, accumulated_width));
            }

            if accumulated_width > max_width {
                break;
            }

            last_fitting_grapheme = Some((grapheme_end, accumulated_width));
        }

        if accumulated_width > max_width {
            break;
        }

        last_fitting_break = Some((break_idx, accumulated_width));
    }

    if let Some(best_break) = last_fitting_break {
        return best_break;
    }

    if let Some(best_grapheme) = last_fitting_grapheme {
        return best_grapheme;
    }

    if let Some(first_grapheme) = first_grapheme {
        return first_grapheme;
    }

    (0, 0.0)
}

fn shaped_width(face: &Face<'_>, text: &str, font_size: f32) -> f32 {
    let mut buffer = UnicodeBuffer::new();
    buffer.push_str(text);
    buffer.guess_segment_properties();

    let shaped = rustybuzz::shape(face, &[], buffer);
    let advance = shaped
        .glyph_positions()
        .iter()
        .fold(0_i32, |acc, pos| acc + pos.x_advance);
    let units_per_em = face.units_per_em() as f32;

    (advance as f32 / units_per_em) * font_size
}

#[cfg(test)]
mod tests {
    use super::{ContainedText, LayoutError, LayoutParams, Metrics, TextLayout, text};

    #[test]
    fn invalid_font_data() {
        let params = LayoutParams {
            font_ttf: b"not-a-valid-font",
            font_size: 12.0,
            max_width: 100.0,
            max_height: 100.0,
            line_height: None,
        };

        let result = text("hello", &params);
        assert_eq!(result, Err(LayoutError::InvalidFont));
    }

    #[test]
    fn empty_text_fits_without_lines() {
        let params = LayoutParams {
            font_ttf: b"not-a-valid-font",
            font_size: 12.0,
            max_width: 100.0,
            max_height: 100.0,
            line_height: None,
        };

        let result = text("", &params);
        assert_eq!(
            result,
            Ok(TextLayout::Contained(ContainedText {
                lines: Vec::new(),
                metrics: Metrics {
                    width: 0.0,
                    height: 0.0,
                },
            }))
        );
    }

    #[test]
    fn non_positive_font_size_is_rejected() {
        let params = LayoutParams {
            font_ttf: b"irrelevant",
            font_size: 0.0,
            max_width: 100.0,
            max_height: 100.0,
            line_height: None,
        };

        let result = text("hello", &params);
        assert_eq!(result, Err(LayoutError::InvalidFontSize));
    }

    #[test]
    fn negative_bounds_are_rejected() {
        let params = LayoutParams {
            font_ttf: b"irrelevant",
            font_size: 12.0,
            max_width: -1.0,
            max_height: 100.0,
            line_height: None,
        };

        let result = text("hello", &params);
        assert_eq!(result, Err(LayoutError::InvalidBounds));
    }
}
