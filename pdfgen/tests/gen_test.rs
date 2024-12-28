use std::path::PathBuf;

use pdfgen::{
    types::hierarchy::primitives::{rectangle::Rectangle, unit::Unit},
    Document,
};

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

#[test]
fn three_pages_different_size() {
    let mut document = Document::builder().with_page_size(Rectangle::A4).build();
    document.create_page();

    let a5_page = document.create_page();
    a5_page.set_mediabox(Rectangle::A5);

    document.create_page();

    macros::snap_test!(document);
}

#[test]
fn page_with_image() {
    let mut document = Document::builder().with_page_size(Rectangle::A4).build();

    let img = document
        .load_image(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sample_image.jpg"))
        .unwrap();

    img.set_dimensions(Unit::from_mm(256.0), Unit::from_mm(256.0));

    let img_ref = img.obj_ref().clone();
    let page = document.create_page();
    page.add_image(img_ref);

    macros::snap_test!(document);
}
