use std::io::Error;

use pdfgen_macros::const_names;

use crate::types::constants;

use super::{
    page_tree::PageTree,
    primitives::{name::Name, obj_id::ObjId, object::Object},
};

/// The root of a document’s object hierarchy, located by means of the `Root` entry in the trailer
/// of the PDF file.
///
/// The catalog dictionary contains references to other objects defining the document’s contents,
/// outline, article threads, named destinations, and other attributes. In addition, it contains
/// information about how the document shall be displayed on the screen, such as whether its
/// outline and thumbnail page images shall be displayed automatically and whether some location
/// other than the first page shall be shown when the document is opened.
#[derive(Debug)]
pub struct Catalog {
    /// The object reference allocated to this `Catalog`.
    id: ObjId,

    /// Reference to the root [`PageTree`] of the PDF Document.
    root_page_tree: PageTree,
}

impl Catalog {
    const_names! {
        CATALOG,
        PAGES,
    }

    /// Create a new `Catalog` with the given [`ObjId`] and [`PageTree`].
    pub(crate) fn new(obj_ref: ObjId, root_page_tree: PageTree) -> Self {
        Self {
            id: obj_ref,
            root_page_tree,
        }
    }

    /// Returns the [`ObjId`] allocated to this `Catalog`.
    pub(crate) fn obj_ref(&self) -> ObjId {
        self.id.clone()
    }

    /// Returns a reference to the root [`PageTree`] that this `Catalog` holds.
    pub(crate) fn page_tree(&self) -> &PageTree {
        &self.root_page_tree
    }

    /// Returns a mutable reference to the root [`PageTree`] that this `Catalog` holds.
    pub(crate) fn page_tree_mut(&mut self) -> &mut PageTree {
        &mut self.root_page_tree
    }
}

impl Object for Catalog {
    fn write_def(&self, writer: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        Ok(pdfgen_macros::write_chain! {
            self.id.write_def(writer),
            writer.write(constants::NL_MARKER),
        })
    }

    fn write_content(&self, writer: &mut dyn std::io::Write) -> Result<usize, Error> {
        let written = pdfgen_macros::write_chain! {
            writer.write(b"<< "),

            Name::TYPE.write(writer),
            Self::CATALOG.write(writer),
            writer.write(constants::NL_MARKER),

            Self::PAGES.write(writer),
            self.root_page_tree.obj_ref().write_ref(writer),

            writer.write(b" >>"),
            writer.write(constants::NL_MARKER),
        };

        Ok(written)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::hierarchy::{
        page_tree::PageTree,
        primitives::{obj_id::IdManager, object::Object},
    };

    use super::Catalog;

    #[test]
    fn simple_catalog() {
        let mut id_manager = IdManager::default();
        let page_tree = PageTree::new(id_manager.create_id(), None);
        let catalog = Catalog::new(id_manager.create_id(), page_tree);

        let mut writer = Vec::default();
        catalog.write_content(&mut writer).unwrap();

        let output = String::from_utf8(writer).unwrap();
        insta::assert_snapshot!(output, @r"
        << /Type /Catalog 
        /Pages 1 0 R >>
        ");
    }
}
