use std::io::Error;

use crate::types::{self, constants};

use super::primitives::{name::Name, obj_ref::ObjRef, object::Object};

pub struct Catalog {
    root_page_tree: ObjRef,
}

impl Catalog {
    const CATALOG: Name = Name::new(b"Catalog");
    const PAGES: Name = Name::new(b"Pages");

    pub(crate) fn new(root_page_tree: ObjRef) -> Self {
        Self { root_page_tree }
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
            self.root_page_tree.write_ref(writer),

            writer.write(b" >>"),
        };

        Ok(written)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::hierarchy::primitives::{obj_ref::ObjRef, object::Object};

    use super::Catalog;

    #[test]
    fn simple_catalog() {
        let catalog = Catalog::new(ObjRef::from(10));

        let mut writer = Vec::default();
        catalog.write(&mut writer).unwrap();

        let output = String::from_utf8(writer).unwrap();
        insta::assert_snapshot!(output, @r#"
        << /Type /Catalog 
        /Pages 10 0 R >>
        "#);
    }
}
