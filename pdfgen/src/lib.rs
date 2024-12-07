#![forbid(unsafe_code)]

//! `pdfgen` is a low-level library that offers fine-grained control over PDF syntax and
//! PDF file generation.

use std::io::{self, Write};
use types::{
    hierarchy::{catalog::Catalog, page_tree::PageTree, primitives::obj_id::IdManager},
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
        }
    }
}

impl Document {
    /// Write the PDF contents into the provided writer.
    pub fn write(&self, writer: &mut impl Write) -> Result<(), io::Error> {
        let mut pdf_writer = PdfWriter::new(writer);
        pdf_writer.write_header()?;

        pdf_writer.write_object(&self.catalog, self.catalog.obj_ref())?;
        pdf_writer.write_object(self.catalog.page_tree(), self.catalog.page_tree().obj_ref())?;

        pdf_writer.write_crt()?;
        pdf_writer.write_trailer(self.catalog.obj_ref())?;
        pdf_writer.write_eof()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Document;

    #[test]
    fn simple_document() {
        let document = Document::default();

        let mut writer = Vec::default();
        document.write(&mut writer).unwrap();

        let output = String::from_utf8(writer).unwrap();

        insta::assert_snapshot!(output, @r#"
        %PDF-2.0
        0 0 obj
        << /Type /Catalog 
        /Pages 1 0 R >>
        endobj
        1 0 obj
        << /Type /Pages 
        /Kids []
        /Count 0 >>
        endobj
        xref
        0 2
        0000000009 00000 n 
        0000000059 00000 n 
        %%EOF
        "#);
    }
}
