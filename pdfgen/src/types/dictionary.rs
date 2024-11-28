#![allow(dead_code)]

use std::{
    borrow::Cow,
    io::{Error, Write},
    str::FromStr,
};

use crate::WriteExt;

pub struct Name(String);

impl Name {
    fn write(&self, writer: &mut impl Write) -> Result<usize, Error> {
        let mut written = writer.write(b"/")?;
        written += writer.write(self.0.as_bytes())?;
        written += writer.write(b" ")?;
        Ok(written)
    }
}

impl FromStr for Name {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value
            .chars()
            .any(|ch| !ch.is_ascii() || ch.is_ascii_whitespace() || ch == '\0')
        {
            return Err(format!("Invalid name: {value}"));
        }

        Ok(Name(value.into()))
    }
}

#[derive(Debug, Clone)]
pub enum DictEntry {
    Version(f64),
}

impl DictEntry {
    pub fn write(&self, writer: &mut impl Write) -> Result<usize, Error> {
        let mut written = 0;

        match self {
            DictEntry::Version(version) => {
                written += writer.write(b"/Version ")?;
                written += writer.write(version.to_string().as_bytes())?;
            }
        }

        Ok(written)
    }
}

struct ObjRef;
struct Rectangle;

impl WriteDictValue for Rectangle {
    fn write(&self, _writer: &mut impl Write) -> Result<usize, Error> {
        todo!()
    }
}

enum PageResource {
    Font { name: String, reference: ObjRef },

    Image(ObjRef),
}

fn _bla() {
    let _page = Page {
        mediabox: Rectangle,
        resources: Vec::new(),
    };

    /*
        <<
          /Type     null
          /Version  1.0
        >>
    * */

    // zaboravis type, ili subtype, ili resources
    // ili neki drugi required za page
    // dictionary.push(DictEntry::Version(1.0));
}

struct Page {
    // type = /Page
    // subtype = /NormalPage (not Template)
    // /Resources << /Font /F1 0 0 3 >>
    mediabox: Rectangle,
    resources: Vec<PageResource>,
}

pub(crate) struct Key(&'static [u8]);

impl Key {
    const TYPE: Key = Key::new(b"Type");

    const fn new<const N: usize>(inner: &'static [u8; N]) -> Self {
        if N == 0 {
            panic!("Dictionary Key must start with '/' followed by at least one ASCII character.");
        }

        let mut i = 0;

        while i < N {
            if inner[i] == b'/' {
                panic!("Dictionary Key is not allowed to contain '/'.");
            }

            i += 1;
        }

        Self(inner)
    }

    pub(crate) fn write(&self, writer: &mut impl Write) -> Result<usize, Error> {
        let mut written = writer.write(b"/")?;
        written += writer.write(self.0)?;
        Ok(written)
    }
}

impl WriteDictValue for Key {
    fn write(&self, writer: &mut impl Write) -> Result<usize, Error> {
        self.write(writer)
    }
}

impl Page {
    const MEDIABOX: Key = Key::new(b"MediaBox");
    const RESOURCES: Key = Key::new(b"MediaBox");

    fn write(&self, writer: &mut impl Write) -> Result<usize, Error> {
        let mut written = writer.write_dict_entry(Key::TYPE, &Key::new(b"Page"))?;
        written += writer.write_dict_entry(Self::MEDIABOX, &self.mediabox)?;

        Ok(written)
    }
}

pub trait AsDictValue {
    fn as_dict_value(&self) -> Cow<'static, [u8]>;
}

pub trait WriteDictValue {
    fn write(&self, writer: &mut impl Write) -> Result<usize, Error>;
}

/// A dictionary object is an associative table containing pairs of objects, known as the
/// dictionary’s entries. The first element of each entry is the key and the second element is the
/// value. The key shall be a name. The value may be any kind of object, including another
/// dictionary.
#[derive(Debug, Clone)]
pub struct Dictionary<T, S = ()> {
    // header: T,
    typ: T,
    subtype: Option<S>,

    entries: Vec<DictEntry>,
}

impl AsDictValue for () {
    fn as_dict_value(&self) -> Cow<'static, [u8]> {
        b"".into()
    }
}

impl<T> Dictionary<T, ()> {
    pub fn new(typ: T) -> Dictionary<T, ()>
    where
        T: AsDictValue,
    {
        Dictionary {
            typ,
            subtype: None,
            entries: Vec::default(),
        }
    }
}

impl<T1, T2> Dictionary<T1, T2> {
    pub fn with_subtype<S>(self, subtype: S) -> Dictionary<T1, S>
    where
        S: AsDictValue,
    {
        Dictionary {
            typ: self.typ,
            subtype: Some(subtype),
            entries: self.entries,
        }
    }

    pub fn push(&mut self, entry: DictEntry) {
        self.entries.push(entry);
    }

    pub fn push_entries(&mut self, entries: impl IntoIterator<Item = DictEntry>) {
        self.entries.extend(entries)
    }

    pub fn write(&self, writer: &mut impl Write) -> Result<usize, Error>
    where
        T1: AsDictValue,
        T2: AsDictValue,
    {
        let mut written = writer.write(b"<< ")?;
        written += writer.write(b"/Type ")?;
        written += writer.write(&self.typ.as_dict_value())?;

        if let Some(subtype) = &self.subtype {
            written += writer.write_newline()?;
            written += writer.write(b"/Subtype ")?;
            written += writer.write(&subtype.as_dict_value())?;
        }

        for entry in &self.entries {
            written += writer.write_newline()?;
            written += writer.write(b"   ")?;
            written += entry.write(writer)?;
        }

        written += writer.write(b" >>")?;

        Ok(written)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::dictionary::DictEntry;

    use super::{AsDictValue, Dictionary};

    struct DummyType;

    impl AsDictValue for DummyType {
        fn as_dict_value(&self) -> std::borrow::Cow<'static, [u8]> {
            b"/DummyType".into()
        }
    }

    #[test]
    fn basic_dictionary() {
        let mut dictionary = Dictionary::new(DummyType);

        dictionary.push(DictEntry::Version(1.1));

        let mut writer = Vec::new();
        let bytes_written = dictionary.write(&mut writer).unwrap();
        let output = String::from_utf8(writer).unwrap();

        assert_eq!(bytes_written, output.len());

        println!("\nOutput: \n\n{output}\n");
    }
}
