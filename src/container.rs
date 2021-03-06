use std::{
    io::{self, BufWriter, Read, Write},
    mem,
};

pub mod header;
pub use header::*;

/// web-sys implementation of the container
#[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
mod browser_impl;
#[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
pub use browser_impl::*;

/// implementation of container if STD is implemented
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
mod desktop_impl;
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
pub use desktop_impl::*;

use super::*;

impl WebContainer {
    /// # Description
    /// packs files and writes into `output`
    /// ## Example
    /// ```
    /// use std::fs;
    /// use web_container::*;
    /// let file = fs::File::create("./test.wpack").unwrap();
    /// Container::pack_to(
    ///     file,
    ///     &mut [
    ///         PackableFile::new(
    ///             "container.rs",
    ///             &mut fs::File::open("./src/container.rs").unwrap(),
    ///         ),
    ///         PackableFile::new(
    ///             "lib.rs",
    ///             &mut fs::File::open("./src/lib.rs").unwrap(),
    ///         ),
    ///     ],
    /// )
    /// .unwrap();
    /// ```
    pub fn pack_to<'a, Memory: Write + Read>(
        output: Memory,
        files: &mut [PackableFile],
    ) -> ContainerError<()> {

        let mut header = Header::default();
        let mut output = BufWriter::new(output);

        const BYTES_OF_NUM_ENTRIES_AND_HEADER_SIZE_AND_TOTAL_SIZE_MEMBERS: usize = 24;
        let header_size = (files.len() * HEADER_ENTRY_IN_BYTES
            + BYTES_OF_NUM_ENTRIES_AND_HEADER_SIZE_AND_TOTAL_SIZE_MEMBERS)
            as u64;

        header.num_entries = files.len() as u64;
        header.header_size = header_size;

        let total_size_of_files =
            files
                .iter()
                .fold(0, |offset, PackableFile { filename, data }| {
                    header.entries.push(HeaderEntry::new(
                        filename,
                        data.len(),
                        offset + header_size,
                    ));
                    offset + data.len()
                });

        header.total_size = total_size_of_files + header.header_size;

        //write header to mem
        header
            .save_to(&mut output)
            .ok_or(ErrorKind::PackFailed("header failed to save"))?;

        //write files to mem
        for file in files.iter_mut() {
            io::copy(file.data, &mut output)
                .map_err(|_| ErrorKind::PackFailed("file copy failed"))?;
        }
        Ok(())
    }
}
