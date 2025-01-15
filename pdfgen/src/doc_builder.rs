//! Types for easier construction of a PDF [`Document`].

use crate::{
    types::hierarchy::{
        catalog::Catalog,
        page_tree::PageTree,
        primitives::{obj_id::IdManager, rectangle::Rectangle},
    },
    Document,
};

/// Used for construction of a PDF [`Document`], enabling streamlined configuration of the
/// document, such as default page size and other options.
pub struct Builder {
    pub(crate) id_manager: IdManager,
    pub(crate) page_size: Option<Rectangle>,
}

impl Builder {
    /// Set the default page size in the document.
    pub fn with_page_size(self, media_box: impl Into<Rectangle>) -> Self {
        Self {
            page_size: Some(media_box.into()),
            ..self
        }
    }

    /// Produce a configured PDF [`Document`].
    pub fn build(mut self) -> Document {
        let catalog_id = self.id_manager.create_id();
        let mut root_page_tree = PageTree::new(self.id_manager.create_id(), None);

        if let Some(rect) = self.page_size {
            root_page_tree.set_page_size(rect);
        }

        let catalog = Catalog::new(catalog_id, root_page_tree);

        Document {
            catalog,
            id_manager: self.id_manager,
            pages: Vec::default(),
            objs: Vec::default(),
            fonts: Vec::default(),
        }
    }
}
