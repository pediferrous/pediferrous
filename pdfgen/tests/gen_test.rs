use std::{fs::File, path::PathBuf};

use pdfgen::{
    Document,
    types::hierarchy::{
        content::{
            color::{CmykValue, Color},
            image::Image,
            text::Text,
        },
        primitives::{
            rectangle::{Position, Rectangle},
            unit::Unit,
        },
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

#[test]
fn page_colored_text() {
    let mut document = Document::builder().with_page_size(Rectangle::A4).build();

    let font_id = document.create_font("Type1".into(), "Helvetica".into());
    let page = document.create_page();

    let pos = Position::from_units(
        Rectangle::A4.width().into_user_unit() / 2.,
        Rectangle::A4.height().into_user_unit() / 2.,
    );
    let builder = Text::builder()
        .with_content("Hello from pdfgen!")
        .with_size(14)
        .with_color(Color::Rgb {
            red: 255,
            green: 0,
            blue: 0,
        })
        .at(pos);

    let red_text = builder.clone().build();
    page.add_text(red_text, font_id.clone());

    let green_text = builder
        .clone()
        .with_color(Color::Rgb {
            red: 0,
            green: 255,
            blue: 0,
        })
        .at(Position {
            x: pos.x,
            y: pos.y + Unit::from_mm(20.),
        })
        .build();
    page.add_text(green_text, font_id.clone());

    let blue_text = builder
        .clone()
        .with_color(Color::Rgb {
            red: 0,
            green: 0,
            blue: 255,
        })
        .at(Position {
            x: pos.x,
            y: pos.y + Unit::from_mm(40.),
        })
        .build();
    page.add_text(blue_text, font_id.clone());

    let yellow_text = builder
        .clone()
        .with_color(Color::Rgb {
            red: 255,
            green: 255,
            blue: 0,
        })
        .at(Position {
            x: pos.x,
            y: pos.y + Unit::from_mm(60.),
        })
        .build();
    page.add_text(yellow_text, font_id.clone());

    let magenta_text = builder
        .clone()
        .with_color(Color::Rgb {
            red: 255,
            green: 0,
            blue: 255,
        })
        .at(Position {
            x: pos.x,
            y: pos.y + Unit::from_mm(80.),
        })
        .build();
    page.add_text(magenta_text, font_id);

    macros::snap_test!(document);
}

#[test]
fn multi_color_space_text() {
    let mut document = Document::builder().with_page_size(Rectangle::A4).build();

    let font_id = document.create_font("Type1".into(), "Helvetica".into());
    let page = document.create_page();

    let pos = Position::from_units(
        Rectangle::A4.width().into_user_unit() / 2.,
        Rectangle::A4.height().into_user_unit() / 2.,
    );

    let color = Color::Rgb {
        red: 255,
        green: 0,
        blue: 0,
    };

    let builder = Text::builder()
        .with_content("Hello from pdfgen!")
        .with_size(14)
        .at(pos);

    let mut with_col_and_offs = |color: Color, offset_in_mm: f32| {
        let text = builder
            .clone()
            .with_color(color)
            .at(Position {
                x: pos.x,
                y: pos.y + Unit::from_mm(offset_in_mm),
            })
            .build();
        page.add_text(text, font_id.clone());
    };

    with_col_and_offs(color, 0.);
    with_col_and_offs(color.to_cmyk(), 5.);

    let gray = Color::Gray(120);
    with_col_and_offs(gray, 15.);
    with_col_and_offs(gray.to_rgb(), 20.);
    with_col_and_offs(gray.to_cmyk(), 25.);

    let pediferrous_blue = Color::CMYK {
        cyan: CmykValue::from_const::<36>(),
        magenta: CmykValue::from_const::<9>(),
        yellow: CmykValue::from_const::<0>(),
        black: CmykValue::from_const::<21>(),
    };
    with_col_and_offs(pediferrous_blue, 45.);
    with_col_and_offs(pediferrous_blue.to_rgb(), 50.);

    macros::snap_test!(document);
}
