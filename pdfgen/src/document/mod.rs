use std::io::{Error, Write};

use crate::types::{
    hierarchy::{catalog::Catalog, page_tree::PageTree, primitives::font::Font},
    page::Page,
    pdf_writer::PdfWriter,
};

mod builder;
pub use builder::Builder;

mod obj_id;
pub(crate) use obj_id::{IdManager, ObjId};

/// This represents one cohesive PDF document that can contain multiple pages of content.
pub struct Document {
    /// The [`Catalog`] PDF object that is the root of the document's object hierarchy.
    catalog: Catalog,

    /// [`IdManager`] is tasked with creation of new [`ObjId`]s. This is the single source of truth
    /// regarding [`ObjId`]s, ensuring that every [`ObjId`] is unique.
    ///
    /// [`ObjId`]: types::hierarchy::primitives::obj_id::ObjId
    id_manager: IdManager,

    /// Collection of all pages in this PDF document.
    pages: Vec<Page>,

    /// Collection of all fonts in this PDF document.
    fonts: Vec<Font>,
}

impl Default for Document {
    fn default() -> Self {
        let mut id_manager = IdManager::new();
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
            id_manager: IdManager::new(),
            page_size: None,
        }
    }

    /// Creates a new page inside the document.
    pub fn create_page(&mut self) -> &mut Page {
        let id = self.id_manager.create_id();
        let contents_id = self.id_manager.create_id();
        self.catalog.page_tree_mut().add_page(id.clone());

        self.pages.push(Page::new(
            id,
            contents_id,
            self.catalog.page_tree().obj_ref(),
        ));

        self.pages.last_mut().unwrap()
    }

    /// Creates a new font inside the document.
    pub fn create_font(
        &mut self,
        name: Vec<u8>,
        subtype: Vec<u8>,
        base_type: Vec<u8>,
    ) -> &mut Font {
        let id = self.id_manager.create_id();

        self.fonts.push(Font::new(name, id, subtype, base_type));

        self.fonts.last_mut().unwrap()
    }

    /// Returns a mutable reference to the current page in document.
    pub fn current_page(&mut self) -> Option<&mut Page> {
        self.pages.last_mut()
    }

    /// Write the PDF contents into the provided writer.
    pub fn write(&self, writer: &mut impl Write) -> Result<(), Error> {
        let mut pdf_writer = PdfWriter::new(writer);
        let mut id_manager = self.id_manager.clone();
        pdf_writer.write_header()?;

        pdf_writer.write_object(&self.catalog)?;
        pdf_writer.write_object(self.catalog.page_tree())?;

        let mut content_streams = Vec::new();

        for page in &self.pages {
            pdf_writer.write_page(page, &mut id_manager)?;
            content_streams.push(page.content_stream());
        }

        for cs in content_streams.into_iter().filter(|cs| !cs.is_empty()) {
            pdf_writer.write_object(cs)?;
        }

        for font in &self.fonts {
            // TODO: should this be here or in `Page`? Both?
            pdf_writer.write_object(font)?;
        }

        pdf_writer.write_crt()?;
        pdf_writer.write_trailer(self.catalog.obj_ref())?;
        pdf_writer.write_eof()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Document, types::hierarchy::primitives::rectangle::Rectangle};

    fn create_sample_doc() -> Document {
        let mut document = Document::default();
        document.create_page().set_mediabox(Rectangle::A4);
        document.create_font("TestName".into(), "Type1".into(), "Helvetica".into());

        document
    }

    #[test]
    fn parallel_but_identical() {
        let mut left_doc = Vec::new();
        let mut right_doc = Vec::new();

        let document = create_sample_doc();

        std::thread::scope(|scope| {
            scope.spawn(|| {
                document.write(&mut left_doc).unwrap();
            });

            scope.spawn(|| {
                document.write(&mut right_doc).unwrap();
            });
        });

        let left_output = String::from_utf8_lossy(&left_doc);
        let right_output = String::from_utf8_lossy(&right_doc);

        pretty_assertions::assert_eq!(left_output, right_output);
    }

    #[test]
    fn simple_document() {
        let document = create_sample_doc();

        let mut writer = Vec::default();
        document.write(&mut writer).unwrap();

        let output = String::from_utf8(writer).unwrap();

        insta::assert_snapshot!(output, @r"
        %PDF-2.0
        1 0 obj
        << /Type /Catalog 
        /Pages 2 0 R >>
        endobj

        2 0 obj
        << /Type /Pages 
        /Kids [3 0 R]
        /Count 1 >>
        endobj

        3 0 obj
        << /Type /Page 
        /Parent 2 0 R
        /Resources <<  >>
        /MediaBox [0 0 592.441 839.0551]>>
        endobj


        5 0 obj
        << /Type /Font 
        /Subtype /Type1 
        /BaseFont /Helvetica 
        /Name /TestName 
        >>
        endobj

        xref
        0 4
        0000000010 00000 n 
        0000000061 00000 n 
        0000000120 00000 n 
        0000000220 00000 n 
        trailer
               << /Size 4
               /Root 1 0 R
               /ID [<d4e71dbc01d10dfdd9eb1e094a6f1dcd>
                  <d4e71dbc01d10dfdd9eb1e094a6f1dcd>
                  ]
               >>
        startxref
        311
        %%EOF
        ");
    }
}
