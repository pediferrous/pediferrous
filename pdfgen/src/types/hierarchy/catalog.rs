use std::io::Error;

use crate::types::{self, constants};

use super::{
    page_tree::PageTree,
    primitives::{name::Name, obj_id::ObjId, object::Object},
};

pub struct Catalog {
    /// The object reference allocated to this `Catalog`.
    obj_ref: ObjId,

    /// Reference to the root [`PageTree`] of the PDF Document.
    root_page_tree: PageTree,
}

impl Catalog {
    const CATALOG: Name = Name::new(b"Catalog");
    const PAGES: Name = Name::new(b"Pages");

    pub(crate) fn new(obj_ref: ObjId, root_page_tree: PageTree) -> Self {
        Self {
            obj_ref,
            root_page_tree,
        }
    }

    pub(crate) fn obj_ref(&self) -> ObjId {
        self.obj_ref.clone()
    }

    pub(crate) fn page_tree(&self) -> &PageTree {
        &self.root_page_tree
    }
}

impl Object for Catalog {
    fn write(&self, writer: &mut impl std::io::Write) -> Result<usize, Error> {
        let written = types::write_chain! {
            writer.write(b"<< "),

            Name::TYPE.write(writer),
            Self::CATALOG.write(writer),
            writer.write(constants::NL_MARKER),

            Self::PAGES.write(writer),
            self.root_page_tree.obj_ref().write_ref(writer),

            writer.write(b" >>"),
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
        catalog.write(&mut writer).unwrap();

        let output = String::from_utf8(writer).unwrap();
        insta::assert_snapshot!(output, @r#"
        << /Type /Catalog 
        /Pages 0 0 R >>
        "#);
    }
}
