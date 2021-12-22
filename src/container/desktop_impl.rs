pub use super::*;
use std::{
    fs,
    io::{self, Read, Seek, SeekFrom, Write, BufReader},
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
        let mut file_handler =
        BufReader::new( fs::File::open(path).map_err(|e| ErrorKind::GenericError(Box::new(e)))?);

        let header =
            Header::load(&mut file_handler).ok_or(ErrorKind::GenericTextError("header load failed"))?;

        println!("header:\n{:?}", header);
        self.header = Some(header);
        self.file = Some(file_handler);

    

        Ok(())
    }

    pub async fn get_file<P, Memory>(&mut self, path: P, mut out: Memory) -> Option<u64>
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
                        .map(|(e_pth, p)| e_pth == p)
                        .unwrap_or(false)
                })
            })
            .flatten()
            .zip(self.file.as_mut())
            .and_then(|(hdr, file)| {
                println!("selected header:\n{:?}", hdr);
                file.seek(SeekFrom::Start(hdr.offset())).ok()?;
                let mut buffer = [0; 1024 *2];
                let mut bytes_to_be_read = hdr.file_size() as usize;
                let buffer_len = buffer.len();
                while let Some(bytes_read) = file
                    .read(&mut buffer[0..bytes_to_be_read.min(buffer_len)])
                    .ok()
                {
                    out.write_all(&buffer[0..bytes_read]).ok()?;
                    bytes_to_be_read -= bytes_read;
                    if bytes_to_be_read == 0 {
                        break;
                    }
                }
                Some(hdr.file_size())
            })
    }
}


mod tests {
    #[allow(unused_imports)]
    use crate::{PackableFile, WebContainer};
    #[allow(unused_imports)]
    use std::{
        fs::{self},
        io::Cursor,
    };

    #[test]
    pub fn pack_sanity() {
        let file = fs::File::create("./resources/test.wpack").unwrap();
        WebContainer::pack_to(
            file,
            &mut [
                PackableFile::new("clip.webm", &mut fs::File::open("./resources/clip.webm").unwrap()),
                PackableFile::new("large.webm", &mut fs::File::open("./resources/large.webm").unwrap()),
            ],
        )
        .unwrap();
    }

    #[test]
    pub fn pack_open_sanity() {
        let mut file = WebContainer::new();
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            file.open("./resources/test.wpack").await.unwrap();
        })
    }

    #[test]
    pub fn open_and_read_sanity() {
        let mut file = WebContainer::new();
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            file.open("./resources/test.wpack").await.unwrap();
            let _ = fs::remove_file("./resources/vid.webm");
            let mut memory = fs::File::create("./resources/vid.webm").unwrap();
            file.get_file("large.webm", &mut memory).await;
        })
    }
}