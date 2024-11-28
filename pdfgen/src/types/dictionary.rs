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
                written += writer.write(b"/Type")?;
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
pub struct Dictionary<T, S> {
    typ: T,
    subtype: Option<S>,

    entries: Vec<DictEntry>,
}

impl<T, S> Dictionary<T, S>
where
    T: AsDictValue,
    S: AsDictValue,
{
    pub fn write(&self, writer: &mut impl Write) -> Result<usize, Error> {
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
            written += entry.write(writer)?;
        }

        written += writer.write(b" >>")?;

        Ok(written)
    }
}
