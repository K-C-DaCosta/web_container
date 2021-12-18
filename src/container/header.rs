use super::*; 
pub const HEADER_ENTRY_IN_BYTES: usize = mem::size_of::<HeaderEntry>();

#[test]
pub fn test() {
    println!("Header entry bytes = {} ", HEADER_ENTRY_IN_BYTES);
}

#[derive(Copy, Clone)]
pub struct HeaderEntry {
    filename: [u8; 128],
    /// number of bytes in the file , little-endian
    file_size: u64,
    /// start of the file, relative to the start of the container , little-endian
    offset: u64,
}
impl HeaderEntry {
    pub fn load<R>(mut res: R) -> Option<Self>
    where
        R: Read,
    {
        let mut buffer = [0u8; 8];
        let mut entry = Self::default();
        //read filename
        res.read_exact(&mut entry.filename).ok()?;

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
        res.write_all(self.filename.as_slice()).ok()?;
        res.write_all(self.file_size.to_le_bytes().as_slice())
            .ok()?;
        res.write_all(self.offset.to_le_bytes().as_slice()).ok()?;

        Some(())
    }
}
impl Default for HeaderEntry {
    fn default() -> Self {
        Self {
            filename: [0; 128],
            file_size: 0,
            offset: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Default)]
pub struct ContainerHeader {
    num_entries: u64,
    /// size of the header segment in bytes, includes everything header related
    container_size: u64,
    entries: Vec<HeaderEntry>,
}

impl ContainerHeader {
    pub fn load<R>(mut res: R) -> Option<Self>
    where
        R: Read,
    {
        let mut buffer = [0; 8];

        //read header length
        res.read_exact(&mut buffer).ok()?;
        let num_entries = u64::from_le_bytes(buffer);

        //container size length
        res.read_exact(&mut buffer).ok()?;
        let container_size = u64::from_le_bytes(buffer);

        let mut container_header = Self {
            num_entries,
            container_size,
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
        res.write_all(&self.container_size.to_le_bytes()).ok()?;
        //load entries byte-by-byte
        for e in &self.entries {
            e.save_to(&mut res)?;
        }
        Some(())
    }
}
