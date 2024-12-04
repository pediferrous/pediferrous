use crate::types::{self, constants, hierarchy::primitives::name::Name};

use super::primitives::{array::WriteArray, obj_ref::ObjRef, object::Object};

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
#[derive(Debug, Default, Clone)]
pub struct PageTree {
    /// The page tree node that is the immediate parent of this one. Required for all nodes except
    /// the root node.
    parent: Option<ObjRef>,

    /// An array of indirect references to the immediate children of this node. The children shall
    /// only be [`Page`] objects or other [`PageTree`] nodes.
    ///
    /// [`Page`]: super::page::Page
    kids: Vec<ObjRef>,

    /// The number of leaf nodes ([`Page`] objects) that are descendants of this node within the
    /// [`PageTree`]
    ///
    /// [`Page`]: super::page::Page
    count: usize,
}

impl PageTree {
    const PARENT: Name = Name::new(b"Parent");
    const PAGES_TYPE: Name = Name::new(b"Pages");
    const KIDS: Name = Name::new(b"Kids");
    const COUNT: Name = Name::new(b"Count");

    pub fn add_page(&mut self, page: ObjRef) {
        self.kids.push(page);
    }
}

impl Object for PageTree {
    fn write(&self, writer: &mut impl std::io::Write) -> Result<usize, std::io::Error> {
        let mut written = types::write_chain! {
            writer.write(b"<< "),
            Name::TYPE.write(writer),
            Self::PAGES_TYPE.write(writer),
            writer.write(constants::NL_MARKER),
        };

        if let Some(parent) = &self.parent {
            written += types::write_chain! {
                Self::PARENT.write(writer),
                parent.write_ref(writer),
                writer.write(constants::NL_MARKER),
            };
        }

        let indent = b"       ".len();
        written += types::write_chain! {
            Self::KIDS.write(writer),
            self.kids.write_array(writer, Some(indent)),
            writer.write(constants::NL_MARKER),

            Self::COUNT.write(writer),
            writer.write(self.count.to_string().as_bytes()),
            writer.write(b" >>"),
        };

        Ok(written)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::hierarchy::primitives::{obj_ref::ObjRef, object::Object};

    use super::PageTree;

    #[test]
    fn simple_page_tree() {
        let page_tree = PageTree::default();

        let mut writer = Vec::new();
        page_tree.write(&mut writer).unwrap();

        let output = String::from_utf8(writer).unwrap();

        insta::assert_snapshot!(output, @r#"
        << /Type /Pages 
        /Kids []
        /Count 0 >>
        "#);
    }

    #[test]
    fn page_tree_with_kids() {
        let mut page_tree = PageTree::default();

        page_tree.add_page(ObjRef::from(0));
        page_tree.add_page(ObjRef::from(1));
        page_tree.add_page(ObjRef::from(2));

        let mut writer = Vec::new();
        page_tree.write(&mut writer).unwrap();

        let output = String::from_utf8(writer).unwrap();

        insta::assert_snapshot!(output, @r#"
        << /Type /Pages 
        /Kids [0 0 R 
               1 0 R 
               2 0 R]
        /Count 0 >>
        "#);
    }
}
