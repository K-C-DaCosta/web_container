use std::error::Error;

/// generic parts of the container
mod container;

/// web-sys implementation of the container
#[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
mod container_web;
#[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
pub use container_web::*;

/// implementation of container if STD is implemented
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
mod container_desktop;
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
pub use container_desktop::*;

mod packing;
pub use packing::*;

#[derive(Debug)]
pub enum ErrorKind {
    PackFailed(&'static str),
    GenericTextError(&'static str),
    GenericError(Box<dyn Error>),
}

pub type ContainerError<T> = Result<T, ErrorKind>;

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
        let file = fs::File::create("./test.wpack").unwrap();
        WebContainer::pack_to(
            file,
            &mut [
                PackableFile::new("clip.webm", &mut fs::File::open("./clip.webm").unwrap()),
                PackableFile::new("large.webm", &mut fs::File::open("./large.webm").unwrap()),
            ],
        )
        .unwrap();
    }

    #[test]
    pub fn pack_open_sanity() {
        let mut file = WebContainer::new();
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            file.open("./test.wpack").await.unwrap();
        })
    }

    #[test]
    pub fn open_and_read_sanity() {
        let mut file = WebContainer::new();
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            file.open("./test.wpack").await.unwrap();
            let _ = fs::remove_file("vid.webm");
            let mut memory = fs::File::create("vid.webm").unwrap();
            file.read_file("large.webm", &mut memory).await;
        })
    }
}
