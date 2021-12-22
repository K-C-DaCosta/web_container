use std::{
    fmt,
    io::{Read, Write},
    str,
};

use super::*;
pub const HEADER_ENTRY_IN_BYTES: usize = mem::size_of::<HeaderEntry>();

#[test]
pub fn test() {
    println!("Header entry bytes = {} ", HEADER_ENTRY_IN_BYTES);
}

#[derive(Copy, Clone)]
pub struct HeaderEntry {
    file_path: [u8; 128],
    /// number of bytes in the file , little-endian
    file_size: u64,
    /// start of the file, relative to the start of the container , little-endian
    offset: u64,
}
impl HeaderEntry {
    pub fn new(filename_arg: &str, file_size: u64, offset: u64) -> Self {
        let mut filename = [0u8; 128];

        filename_arg
            .as_bytes()
            .iter()
            .zip(filename.iter_mut())
            .for_each(|(&c, stack_c)| {
                *stack_c = c;
            });

        Self {
            file_path: filename,
            file_size,
            offset,
        }
    }
    pub fn load<R>(mut res: R) -> Option<Self>
    where
        R: Read,
    {
        let mut buffer = [0u8; 8];
        let mut entry = Self::default();
        //read filename
        res.read_exact(&mut entry.file_path).ok()?;

        //read filesize
        res.read_exact(&mut buffer).ok()?;
        entry.file_size = u64::from_le_bytes(buffer);

        //read offset
        res.read_exact(&mut buffer).ok()?;
        entry.offset = u64::from_le_bytes(buffer);

        Some(entry)
    }

    pub fn save_to<R>(&self, mut res: R) -> Option<()>
    where
        R: Write,
    {
        res.write_all(self.file_path.as_slice()).ok()?;
        res.write_all(self.file_size.to_le_bytes().as_slice())
            .ok()?;
        res.write_all(self.offset.to_le_bytes().as_slice()).ok()?;

        Some(())
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn file_path(&self) -> &[u8] {
        &self.file_path
    }
    pub fn file_path_as_str(&self) -> Option<&str> {
        str::from_utf8(self.file_path())
            .ok()
            .and_then(|s| s.find('\0').map(|n| &s[0..n]).or(Some(s)))
            .map(|s| {
                println!("{:?}", s);
                s
            })
    }
    pub fn file_size(&self) -> u64 {
        self.file_size
    }
}

impl Default for HeaderEntry {
    fn default() -> Self {
        Self {
            file_path: [0; 128],
            file_size: 0,
            offset: 0,
        }
    }
}

impl fmt::Debug for HeaderEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let filename = str::from_utf8(&self.file_path)
            .ok()
            .and_then(|s| s.find('\0').map(|n| &s[0..n]).or(Some(s)))
            .unwrap();

        let file_size = self.file_size;
        let offset = self.offset;

        write!(
            f,
            "HeaderEntry {{ filename: \"{}\", file_size: {}, offset: {} }}",
            filename, file_size, offset
        )?;

        Ok(())
    }
}

#[repr(C)]
#[derive(Clone, Default, Debug)]
pub struct Header {
    pub num_entries: u64,
    /// size of the header segment in bytes, includes everything header related
    pub header_size: u64,
    /// size of the entire container in bytes
    pub total_size: u64,
    /// contains data about every file in the container
    pub entries: Vec<HeaderEntry>,
}

impl Header {
    pub fn load<Readable>(mut res: Readable) -> Option<Self>
    where
        Readable: Read,
    {
        let mut buffer = [0; 8];

        //read header length
        res.read_exact(&mut buffer).ok()?;
        let num_entries = u64::from_le_bytes(buffer);

        //container size length
        res.read_exact(&mut buffer).ok()?;
        let header_size = u64::from_le_bytes(buffer);

        //container size length
        res.read_exact(&mut buffer).ok()?;
        let total_size = u64::from_le_bytes(buffer);

        let mut container_header = Self {
            num_entries,
            header_size,
            total_size,
            entries: Vec::with_capacity(num_entries as usize),
        };

        //load entries byte-by-byte
        for _ in 0..container_header.num_entries as usize {
            container_header.entries.push(HeaderEntry::load(&mut res)?);
        }

        Some(container_header)
    }

    pub fn save_to<R>(&self, mut res: R) -> Option<()>
    where
        R: Write,
    {
        res.write_all(&self.num_entries.to_le_bytes()).ok()?;
        res.write_all(&self.header_size.to_le_bytes()).ok()?;
        res.write_all(&self.total_size.to_le_bytes()).ok()?;
        //load entries byte-by-byte
        for e in &self.entries {
            e.save_to(&mut res)?;
        }
        Some(())
    }
}
