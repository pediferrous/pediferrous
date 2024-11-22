#![allow(dead_code)]

use std::collections::HashMap;

pub enum Boolean {
    True,
    False,
}

pub struct Integer(isize);
pub struct Real(f64);
pub struct PdfString(String);
pub struct Name(String);
pub struct Array<T>(Vec<T>);
pub struct Dictionary<K, V>(HashMap<K, V>);
pub struct Stream;
pub struct Null;

pub struct Object {}
