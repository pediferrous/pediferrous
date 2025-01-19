use pdfgen_macros::const_names;

use crate::types::{constants, hierarchy::primitives::name::Name};

use super::primitives::{array::WriteArray, obj_id::ObjId, object::Object, rectangle::Rectangle};

/// Page tree is a structure which defines the ordering of pages in the document. The tree contains
/// nodes of two types:
///
/// * intermediate nodes, which are [`PageTree`] nodes
/// * leaf nodes, which are [`Page`] objects.
///
/// The simplest structure can consist of a single page tree node that references all of the
/// document’s page objects directly. However, to optimise application performance, a PDF writer
/// can construct trees of a particular form, known as balanced trees.
///
/// [`Page`]: super::page::Page
#[derive(Debug, Clone)]
pub struct PageTree {
    /// The object reference allocated for this `PageTree`.
    obj_id: ObjId,

    /// The page tree node that is the immediate parent of this one. Required for all nodes except
    /// the root node.
    parent: Option<ObjId>,

    /// An array of indirect references to the immediate children of this node. The children shall
    /// only be [`Page`] objects or other [`PageTree`] nodes.
    ///
    /// [`Page`]: super::page::Page
    kids: Vec<ObjId>,

    /// The number of leaf nodes ([`Page`] objects) that are descendants of this node within the
    /// [`PageTree`]
    ///
    /// [`Page`]: super::page::Page
    count: usize,

    /// Default Mediabox used for all [`Page`]s that are descendants of this `PageTree`.
    ///
    /// [`Page`]: super::page::Page
    default_mediabox: Option<Rectangle>,
}

impl PageTree {
    const_names! {
        PARENT,
        PAGES,
        MEDIA_BOX,
        KIDS,
        COUNT,
    }

    pub fn new(obj_id: ObjId, parent: Option<&PageTree>) -> Self {
        Self {
            obj_id,
            parent: parent.map(|parent| parent.obj_ref()),
            kids: Vec::default(),
            count: 0,
            default_mediabox: None,
        }
    }

    pub fn with_mediabox(
        obj_id: ObjId,
        parent: Option<&PageTree>,
        mediabox: impl Into<Rectangle>,
    ) -> Self {
        let mut page_tree = Self::new(obj_id, parent);
        page_tree.default_mediabox = Some(mediabox.into());
        page_tree
    }

    pub fn add_page(&mut self, page: ObjId) {
        self.kids.push(page);
        self.count += 1;
    }

    pub fn obj_ref(&self) -> ObjId {
        self.obj_id.clone()
    }

    pub(crate) fn set_page_size(&mut self, rect: Rectangle) {
        self.default_mediabox = Some(rect);
    }
}

impl Object for PageTree {
    fn write(&self, writer: &mut dyn std::io::Write) -> Result<usize, std::io::Error> {
        let indent_level = Self::KIDS.len() + constants::SP.len();

        let written = pdfgen_macros::write_chain! {
            writer.write(b"<< "),
            Name::TYPE.write(writer),
            Self::PAGES.write(writer),
            writer.write(constants::NL_MARKER),

            if let Some(parent) = &self.parent {
                Self::PARENT.write(writer),
                parent.write_ref(writer),
                writer.write(constants::NL_MARKER),
            },

            if let Some(mediabox) = self.default_mediabox {
                Self::MEDIA_BOX.write(writer),
                mediabox.write(writer),
                writer.write(constants::NL_MARKER),
            },

            Self::KIDS.write(writer),
            self.kids.write_array(writer, Some(indent_level)),
            writer.write(constants::NL_MARKER),

            Self::COUNT.write(writer),
            writer.write(self.count.to_string().as_bytes()),
            writer.write(b" >>"),
        };

        Ok(written)
    }

    fn obj_ref(&self) -> &ObjId {
        &self.obj_id
    }
}

#[cfg(test)]
mod tests {
    use crate::types::hierarchy::primitives::{obj_id::IdManager, object::Object};

    use super::PageTree;

    #[test]
    fn simple_page_tree() {
        let mut id_manager = IdManager::default();
        let page_tree = PageTree::new(id_manager.create_id(), None);

        let mut writer = Vec::new();
        page_tree.write(&mut writer).unwrap();

        let output = String::from_utf8(writer).unwrap();

        insta::assert_snapshot!(output, @r"
        << /Type /Pages 
        /Kids []
        /Count 0 >>
        ");
    }

    #[test]
    fn page_tree_with_kids() {
        let mut id_manager = IdManager::default();
        let mut page_tree = PageTree::new(id_manager.create_id(), None);

        page_tree.add_page(id_manager.create_id());
        page_tree.add_page(id_manager.create_id());
        page_tree.add_page(id_manager.create_id());

        let mut writer = Vec::new();
        page_tree.write(&mut writer).unwrap();

        let output = String::from_utf8(writer).unwrap();

        insta::assert_snapshot!(output, @r"
        << /Type /Pages 
        /Kids [2 0 R
               3 0 R
               4 0 R]
        /Count 3 >>
        ");
    }
}
