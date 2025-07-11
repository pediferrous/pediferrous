#![forbid(unsafe_code)]

//! `pdfgen` is a low-level library that offers fine-grained control over PDF syntax and
//! PDF file generation.

pub mod types;

mod document;
pub use document::Document;
pub(crate) use document::{IdManager, ObjId};
pub(crate) mod macros;
