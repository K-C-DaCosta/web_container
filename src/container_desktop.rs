pub use super::*;
use std::{
    fs,
    io::{self, Read, Seek, SeekFrom, Write},
    path::Path,
};

pub use crate::container::*;

pub struct WebContainer {
    header: Option<Header>,
    file: Option<io::BufReader<fs::File>>,
}

impl WebContainer {
    pub fn new() -> Self {
        Self {
            header: None,
            file: None,
        }
    }

    pub async fn open<P>(&mut self, path: P) -> ContainerError<()>
    where
        P: AsRef<Path>,
    {
        let file = fs::File::open(path).map_err(|e| ErrorKind::GenericError(Box::new(e)))?;
        let header =
            Header::load(&file).ok_or(ErrorKind::GenericTextError("header load failed"))?;
        println!("header:\n{:?}", header);
        self.header = Some(header);
        self.file = Some(io::BufReader::new(file));
        Ok(())
    }

    pub async fn read_file<P, Memory>(&mut self, path: P, mut out: Memory) -> Option<u64>
    where
        P: AsRef<Path>,
        Memory: Write,
    {
        let path = path.as_ref().to_str();

        self.header
            .as_ref()
            .map(|hdr| {
                hdr.entries.iter().find(|entry| {
                    entry
                        .file_path_as_str()
                        .zip(path)
                        .map(|(e_pth, p)| {
                            e_pth == p
                        })
                        .unwrap_or(false)
                })
            })
            .flatten()
            .zip(self.file.as_mut())
            .and_then(|(hdr, file)| {
                // println!("selected header:\n{:?}", hdr);
                file.seek(SeekFrom::Start(hdr.offset())).ok()?;
                let mut buffer = [0; 1024];
                let mut bytes_to_be_read = hdr.file_size() as usize;
                while let Some(bytes_read) =
                    file.read(&mut buffer[0..bytes_to_be_read.min(1024)]).ok()
                {
                    out.write_all(&buffer[0..bytes_read]).ok()?;
                    bytes_to_be_read -= bytes_read;
                    if bytes_to_be_read == 0 {
                        break;
                    }
                }
                Some(0)
            })
    }
}
