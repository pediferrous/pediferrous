use crate::{
    types::hierarchy::{
        catalog::Catalog,
        page_tree::PageTree,
        primitives::{obj_id::IdManager, rectangle::Rectangle},
    },
    Document,
};

pub struct Builder {
    pub(crate) id_manager: IdManager,
    pub(crate) page_size: Option<Rectangle>,
}

impl Builder {
    pub fn with_page_size(self, media_box: impl Into<Rectangle>) -> Self {
        Self {
            page_size: Some(media_box.into()),
            ..self
        }
    }

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
        }
    }
}
