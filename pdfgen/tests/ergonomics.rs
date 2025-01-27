// #![allow(unused)]
// #![allow(dead_code)]
//
// use std::fs::File;
//
// use pdfgen::{
//     types::hierarchy::{content::image::Image, primitives::rectangle::Position},
//     Document,
// };
//
// #[test]
// fn ergonomic_api() {
//     // create document with title and default page size.
//     // let mut doc = Document::new("Document title", Rectangle::A4);
//     let mut doc = Document::default();
//
//     // add a page with mutable reference to the page
//     let page = doc.create_page();
//
//     // // draw text on page:
//     // // NOTE: we don't support this as of right now anyways
//     // page.draw_text(
//     //     Text::from("Some text")
//     //         .with_font(Font::serif())
//     //         .with_position(Position::from_mm(100., 100.)),
//     // );
//     //
//     // // NOTE: we don't support this as of right now anyways
//     // page.draw_rectangle(
//     //     Rectangle::from((50., 50., 100., 10.)),
//     //     DrawStyle::Fill,
//     //     Color::black(),
//     // );
//
//     let file = File::open("./some/file.png").expect("Failed to read the file.");
//     // page.add_image(
//     //     Image::from_file(file)
//     //         .at(Position::from_mm(40., 40.))
//     //         .scaled(/* width, height */)
//     //         .rotated(/* degree */)
//     //         .build(),
//     // );
//
//     let img = Image::from_file(file)
//         .at(Position::from_mm(10., 10.))
//         .build();
//
//     let mut out_file = File::create("./some/out_file.pdf");
//     // doc.write_to(&mut out_file);
// }
