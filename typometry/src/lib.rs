//! Typometry provides text layout primitives for fitting shaped text into bounding boxes.

pub mod layout;

pub use layout::{ContainedText, LayoutError, LayoutParams, Line, Metrics, TextLayout, text};
