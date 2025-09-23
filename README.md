# Project Structure

This project is organized as a multirepo, consisting of two separate crates:

1. **pdfgen** — a low-level PDF rendering crate that handles the core structure and internals of PDF generation. It focuses on providing precise control over catalogs, page trees, objects, references, and other essential components of PDF files.  
2. **pediferrous** — a planned high-level PDF generation crate that will build on top of `pdfgen`, offering an intuitive and lightweight API for developers who want to create PDFs without dealing with low-level details.

## pdfgen

**pdfgen** is the foundation crate: it provides precise control over PDF structure, as well as content authoring (text, fonts, colors, images). It manages catalogs, page trees, cross-reference tables, trailers, and object references — all the building blocks for generating PDF files programmatically.

This crate is ideal for developers who need fine-grained control or want to build higher-level abstractions like the upcoming **pediferrous** crate.

### Key Features

- **Core PDF Structure Management**: Automatically handles critical PDF components, including:
  - Catalogs
  - Page Trees
  - Object References
  - Cross-Reference Tables (CRT)
  - Trailers
- **Flexible Page creation**
  - Create pages with standard or custom dimensions (`A4`, `A5`, ..., custom `Rectangle`)  
- **Content authoring**
  - Content stream objects  
  - UTF-8 text strings  
  - Font support (start with standard Type1 fonts)  
  - Text objects with positioning & sizing  
  - Superscript and subscript text  
  - Color management (Gray, RGB, CMYK with conversion helpers)  
  - Raster image embedding (position, scaling)  
- **Streamlined output**
  - Write fully compliant PDF files to any `Write` implementation

---

### Getting Started

Add `pdfgen` to your `Cargo.toml`:

```toml
[dependencies]
pdfgen = "0.3.1"
```

---

### Example: full usage

```rust
use std::{fs::File, path::PathBuf, io::Result};

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

fn main() -> Result<()> {
    // Create document with A4 page size
    let mut document = Document::builder().with_page_size(Rectangle::A4).build();

    // Register a standard font
    let font_id = document.create_font("Type1".into(), "Helvetica".into());

    // Create a page
    let page = document.create_page();

    // --- Create a color ---
    let red = Color::Rgb {
        red: 255,
        green: 0,
        blue: 0,
    };

    // --- Add a colored text ---
    let colored_text = Text::builder()
        .with_content("Hello ")
        .with_expanded_content("from colorful pdfgen!")
        .with_size(14)
        .at(Position::from_units(
            Rectangle::A4.width().into_user_unit() / 2. - 260.,
            Rectangle::A4.height().into_user_unit() / 2. + 200.,
        ))
        .with_color(red)
        .build();

    page.add_text(colored_text, font_id.clone());

    // --- Add superscript & subscript texts ---
    let presuperscript_text = Text::builder()
        .with_content("Hello, here's a fun formula for you: E=m*c")
        .with_size(14)
        .at(Position::from_units(
            Rectangle::A4.width().into_user_unit() / 2. - 260.,
            Rectangle::A4.height().into_user_unit() / 2.,
        ))
        .build();

    page.add_text(presuperscript_text, font_id.clone());

    let superscript_text = Text::builder()
        .with_content("2")
        .with_size(14)
        .at(Position::from_units(
            Rectangle::A4.width().into_user_unit() / 2.,
            Rectangle::A4.height().into_user_unit() / 2.,
        ))
        .superscript()
        .build();

    page.add_text(superscript_text, font_id.clone());

    let presubscript_text = Text::builder()
        .with_content("How about chemical notation for carbon dioxide, ykyk: CO")
        .with_size(14)
        .at(Position::from_units(
            Rectangle::A4.width().into_user_unit() / 2. - 260.,
            Rectangle::A4.height().into_user_unit() / 2. - 200.,
        ))
        .build();

    page.add_text(presubscript_text, font_id.clone());

    let subcript_text = Text::builder()
        .with_content("2")
        .with_size(14)
        .at(Position::from_units(
            Rectangle::A4.width().into_user_unit() / 2. + 100.,
            Rectangle::A4.height().into_user_unit() / 2. - 200.,
        ))
        .subscript()
        .build();

    page.add_text(subcript_text, font_id.clone());

    // --- Add an image ---
    let img = Image::from_file(&File::open(PathBuf::from("sample_image.jpg")).unwrap())
        .at(Position::from_units(50., 50.))
        .scaled(Position::from_units(100., 100.))
        .build();

    page.add_image(img);

    // --- Write document to file ---
    let mut file = File::create("output.pdf")?;
    doc.write(&mut file)?;
    Ok(())
}
```
