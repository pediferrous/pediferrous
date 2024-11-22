//! General-purpose data structures that are built from the basic object types described in 7.3,
//! "Objects" and used throughout PDF technology. This subclause describes data structures for text
//! strings, dates, rectangles, name trees, and number trees.
//!
//! All of these data structures are meaningful only as part of the document hierarchy; they may
//! not appear within content streams. In particular, the special conventions for interpreting the
//! values of string objects apply only to strings outside content streams. An entirely different
//! convention is used within content streams for using strings to select sequences of glyphs to be
//! painted on the page.
//!
//! Reference: ISO 32000-2:2020 (PDF 2.0); page 114

pub mod string;

/// Null object.
pub struct Null;
