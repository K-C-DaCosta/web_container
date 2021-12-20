use std::{
    fs,
    io::{Cursor, Read, Seek, Write},
};

pub struct PackableFile<'a, 'b> {
    pub filename: &'a str,
    pub data: &'b mut dyn WebPackable,
}
impl<'a, 'b> PackableFile<'a, 'b> {
    pub fn new(filename: &'a str, data: &'b mut dyn WebPackable) -> Self {
        Self { filename, data }
    }
}

pub trait WebPackable: Read + Write + Seek {
    fn len(&self) -> u64;
}

impl WebPackable for fs::File {
    fn len(&self) -> u64 {
        self.metadata().unwrap().len()
    }
}
impl WebPackable for Cursor<Vec<u8>> {
    fn len(&self) -> u64 {
        Self::get_ref(&self).len() as u64
    }
}
