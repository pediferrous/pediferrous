use std::{
    io::{Read, Write},
    path::PathBuf,
};

use pdfgen::{types::hierarchy::primitives::rectangle::Rectangle, Document};

mod macros;

#[test]
fn public_api() {
    let mut document = Document::builder().with_page_size(Rectangle::A5).build();
    document.create_page();

    macros::snap_test!(document);
}

#[test]
fn two_empty_pages() {
    let mut document = Document::builder().with_page_size(Rectangle::A5).build();
    document.create_page();
    document.create_page();

    macros::snap_test!(document);
}
