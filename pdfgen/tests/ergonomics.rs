#![allow(unused)]
#![allow(dead_code)]

use std::{fs::File, path::PathBuf};

use pdfgen::{
    types::hierarchy::{
        content::image::Image,
        primitives::rectangle::{Position, Rectangle},
    },
    Document,
};

mod macros;

#[test]
fn ergonomic_api() {
    // create document with title and default page size.
    // let mut doc = Document::new("Document title", Rectangle::A4);
    let mut doc = Document::builder()
        .with_page_size(Rectangle::from_units(0., 0., 256., 256.))
        .build();

    // add a page with mutable reference to the page
    let page = doc.create_page();

    // // draw text on page:
    // // NOTE: we don't support this as of right now anyways
    // page.draw_text(
    //     Text::from("Some text")
    //         .with_font(Font::serif())
    //         .with_position(Position::from_mm(100., 100.)),
    // );

    // // NOTE: we don't support this as of right now anyways
    // page.draw_rectangle(
    //     Rectangle::from((50., 50., 100., 10.)),
    //     DrawStyle::Fill,
    //     Color::black(),
    // );

    let file =
        File::open(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sample_image.jpg")).unwrap();

    page.add_image(
        Image::from_file(&file)
            .at(Position::from_units(40., 40.))
            .scaled(Position::from_units(120., 120.))
            // NOTE: not supported right now
            // .rotated(/* degree */)
            .build(),
    );

    let mut out_file = File::create("./some/out_file.pdf");

    macros::snap_test!(doc);
}
