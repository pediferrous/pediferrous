#![allow(dead_code)]

use std::{
    borrow::Cow,
    io::{Error, Write},
};

use crate::WriteExt;

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

pub trait AsDictValue {
    fn as_dict_value(&self) -> Cow<'static, [u8]>;
}

#[derive(Debug, Clone)]
pub struct Dictionary<T, S = ()> {
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
