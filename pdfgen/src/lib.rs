#![forbid(unsafe_code)]

//! `pdfgen` is a low-level library that offers fine-grained control over PDF syntax and
//! PDF file generation.

mod doc_builder;

pub use doc_builder::Builder;

use std::io::{self, Write};
use types::{
    hierarchy::{
        catalog::Catalog,
        page_tree::PageTree,
        primitives::{font::Font, obj_id::IdManager},
    },
    page::Page,
    pdf_writer::PdfWriter,
};

pub mod types;

/// This represents one cohesive PDF document that can contain multiple pages of content.
pub struct Document {
    /// The [`Catalog`] PDF object that is the root of the document's object hierarchy.
    catalog: Catalog,

    /// [`IdManager`] is tasked with creation of new [`ObjId`]s. This is the single source of truth
    /// regarding [`ObjId`]s, ensuring that every [`ObjId`] is unique.
    ///
    /// [`ObjId`]: types::hierarchy::primitives::obj_id::ObjId
    #[allow(dead_code)]
    id_manager: IdManager,

    /// Collection of all pages in this PDF document.
    pages: Vec<Page>,

    /// Collection of all fonts in this PDF document.
    fonts: Vec<Font>,
}

impl Default for Document {
    fn default() -> Self {
        let mut id_manager = IdManager::default();
        let catalog_id = id_manager.create_id();
        let page_tree_root_id = id_manager.create_id();
        let root_page_tree = PageTree::new(page_tree_root_id, None);
        let catalog = Catalog::new(catalog_id, root_page_tree);

        Self {
            catalog,
            id_manager,
            pages: Vec::new(),
            fonts: Vec::new(),
        }
    }
}

impl Document {
    pub fn builder() -> Builder {
        Builder {
            id_manager: IdManager::default(),
            page_size: None,
        }
    }

    /// Creates a new page inside the document.
    pub fn create_page(&mut self) -> &mut Page {
        let id = self.id_manager.create_id();
        self.catalog.page_tree_mut().add_page(id.clone());

        self.pages
            .push(Page::new(id, self.catalog.page_tree().obj_ref()));

        self.pages.last_mut().unwrap()
    }

    /// Creates a new font inside the document.
    pub fn create_font(&mut self, subtype: Vec<u8>, base_type: Vec<u8>) -> &mut Font {
        let id = self.id_manager.create_id();

        self.fonts.push(Font::new(id, subtype, base_type));

        self.fonts.last_mut().unwrap()
    }

    /// Write the PDF contents into the provided writer.
    pub fn write(&self, writer: &mut impl Write) -> Result<(), io::Error> {
        let mut pdf_writer = PdfWriter::new(writer);
        pdf_writer.write_header()?;

        pdf_writer.write_object(&self.catalog, self.catalog.obj_ref())?;
        pdf_writer.write_object(self.catalog.page_tree(), self.catalog.page_tree().obj_ref())?;

        // Write pages
        for page in &self.pages {
            pdf_writer.write_object(page, page.obj_ref())?;
        }

        // Write fonts
        for font in &self.fonts {
            pdf_writer.write_object(font, font.obj_ref())?;
        }

        pdf_writer.write_crt()?;
        pdf_writer.write_trailer(self.catalog.obj_ref())?;
        pdf_writer.write_eof()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{types::hierarchy::primitives::rectangle::Rectangle, Document};

    #[test]
    fn simple_document() {
        let mut document = Document::default();
        document.create_page().set_mediabox(Rectangle::A4);
        document.create_font("Type1".into(), "Helvetica".into());

        let mut writer = Vec::default();
        document.write(&mut writer).unwrap();

        let output = String::from_utf8(writer).unwrap();

        insta::assert_snapshot!(output, @r"
        %PDF-2.0
        0 0 obj
        << /Type /Catalog 
        /Pages 1 0 R >>
        endobj
        1 0 obj
        << /Type /Pages 
        /Kids [2 0 R]
        /Count 1 >>
        endobj
        2 0 obj
        << /Type /Page /Parent 1 0 R /Resources <<  >> /MediaBox [0 0 592.441 839.0551] >>
        endobj
        3 0 obj
        << /Type /Font 
        /Subtype /Type1 
        /BaseFont /Helvetica 
        >>
        endobj
        xref
        0 4
        0000000009 00000 n 
        0000000059 00000 n 
        0000000117 00000 n 
        0000000215 00000 n 
        trailer
               << /Size 4
               /Root 0 0 R
               /ID [<d7336a410f3f6b8c3f65b8105453493d>
                  <d7336a410f3f6b8c3f65b8105453493d>
                  ]
               >>
        startxref
        288
        %%EOF
        ");
    }
}
