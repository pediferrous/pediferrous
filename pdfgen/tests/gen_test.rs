use std::{fs::File, path::PathBuf};

use pdfgen::{
    Document,
    types::hierarchy::{
        content::{image::Image, text::Text},
        primitives::rectangle::{Position, Rectangle},
    },
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
    let page_size = 64.;
    let mut document = Document::builder()
        .with_page_size(Rectangle::from_units(0., 0., page_size, page_size))
        .build();

    let page = document.create_page();

    let img = Image::from_file(
        &File::open(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sample_image.jpg")).unwrap(),
    )
    .at(Position::from_units(0., 0.))
    .build();

    page.add_image(img);

    document.current_page();

    macros::snap_test!(document);
}

#[test]
fn page_image_moved() {
    let page_size = 128.;
    let mut document = Document::builder()
        .with_page_size(Rectangle::from_units(0., 0., page_size, page_size))
        .build();

    let page = document.create_page();

    let img = Image::from_file(
        &File::open(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sample_image.jpg")).unwrap(),
    )
    .at(Position::from_units(64. - 88. / 2., 13.))
    .build();

    page.add_image(img);

    document.current_page();

    macros::snap_test!(document);
}

#[test]
fn page_image_moved_and_scaled() {
    let page_size = 128.;
    let mut document = Document::builder()
        .with_page_size(Rectangle::from_units(0., 0., page_size, page_size))
        .build();

    let page = document.create_page();

    let img = Image::from_file(
        &File::open(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sample_image.jpg")).unwrap(),
    )
    .at(Position::from_units(64. - 88. / 2., 13.))
    .scaled(Position::from_units(88., 88.))
    .build();

    page.add_image(img);

    macros::snap_test!(document);
}

#[test]
fn page_text() {
    let mut document = Document::builder().with_page_size(Rectangle::A4).build();

    let font_id = document.create_font("Type1".into(), "Helvetica".into());
    let page = document.create_page();

    let txt = Text::builder()
        .with_content("Hello ")
        .with_expanded_content("from pdfgen!")
        .with_size(14)
        .at(Position::from_units(
            Rectangle::A4.width().into_user_unit() / 2.,
            Rectangle::A4.height().into_user_unit() / 2.,
        ))
        .build();

    page.add_text(txt, font_id);

    macros::snap_test!(document);
}
