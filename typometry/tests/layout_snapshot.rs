use std::{fs::File, io::Read};

use typometry::{LayoutParams, TextLayout};

const SNAPSHOT_FONT_PATH: &str = concat!(
    std::env!("CARGO_MANIFEST_DIR"),
    "/tests/fonts/aileron/Aileron-Regular.otf"
);

#[test]
fn layout_breaks_text_into_expected_lines() {
    let font_ttf = load_font_bytes(SNAPSHOT_FONT_PATH);
    let text = "This snapshot verifies that typometry layout breaks text into lines in a stable and inspectable way.";

    let params = LayoutParams {
        font_ttf: &font_ttf,
        font_size: 14.0,
        max_width: 170.0,
        max_height: 1_000.0,
        line_height: None,
    };

    let layout = typometry::text(text, &params).expect("layout parameters are valid");

    let snapshot = render_layout_lines(&layout);
    insta::assert_snapshot!("layout_breaks_text_into_expected_lines", snapshot);
}

fn load_font_bytes(path: &str) -> Vec<u8> {
    let mut file = File::open(path).expect("font file exists");
    let mut font_ttf = Vec::new();
    file.read_to_end(&mut font_ttf)
        .expect("font file is readable");

    font_ttf
}

fn render_layout_lines(layout: &TextLayout<'_>) -> String {
    match layout {
        TextLayout::Contained(contained) => {
            let mut out = String::new();
            out.push_str("status: contained\n");
            out.push_str("non-empty lines:\n");
            write_non_empty_lines(&mut out, &contained.lines);
            out
        }
        TextLayout::Overflow {
            contained,
            remaining,
        } => {
            let mut out = String::new();
            out.push_str("status: overflow\n");
            out.push_str("non-empty lines:\n");
            write_non_empty_lines(&mut out, &contained.lines);
            out.push_str(&format!("remaining: {:?}\n", remaining));
            out
        }
    }
}

fn write_non_empty_lines(out: &mut String, lines: &[typometry::Line<'_>]) {
    for (idx, line) in lines
        .iter()
        .filter(|line| !line.text.is_empty())
        .enumerate()
    {
        out.push_str(&format!(
            "{idx:02}: {:?} (w={:.3})\n",
            line.text, line.width
        ));
    }
}
