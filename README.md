## Project Structure

This project is organized as a multirepo, consisting of two separate crates:

1. **PDFGen**: A low-level PDF rendering crate that handles the core structure and internals of PDF generation. It focuses on providing precise control over catalogs, page trees, objects, references, and other essential components of PDF files.

2. **Pediferrous**: A planned high-level PDF generation crate that will build on top of `PDFGen`, offering an intuitive and lightweight API for developers who want to create PDFs without dealing with low-level details.


## PDFGen

**PDFGen** is a low-level PDF rendering crate written in Rust, designed to provide precise control over PDF structure creation. It forms the foundation for managing catalogs, page trees, cross-reference tables, trailers, and object references—all of which are essential for generating PDF files programmatically.

This crate is ideal for developers looking to build custom PDF workflows or higher-level abstractions like the upcoming **Pediferrous** crate.

### Key Features

- **Core PDF Structure Management**: Automatically handles critical PDF components, including:
  - Catalogs
  - Page Trees
  - Object References
  - Cross-Reference Tables (CRT)
  - Trailers
- **Flexible Page Creation**: Easily add pages with customizable dimensions.
- **Streamlined Output**: Write fully compliant PDF files to any `Write` implementation.

### Getting Started

To include `pdfgen` in your project, add it to your `Cargo.toml`:

```toml
[dependencies]
pdfgen = "0.2.0"
```

### Example Usage
```rust
use pdfgen::{types::hierarchy::primitives::rectangle::Rectangle, Document};

use std::fs::File;
use std::io::Result;

fn main() -> Result<()> {
    let mut document = Document::default();

    // Create a new page with a specific size (e.g., A4 dimensions in points).
    let page = document.create_page(Rectangle::from((0, 0, 595, 842)));

    // Modify the page if needed (e.g., add text or graphics).
    // Note: Low-level content APIs will be implemented in future updates.

    // Write the PDF to a file.
    let mut file = File::create("output.pdf")?;
    document.write(&mut file)?;

    Ok(())
}
```
