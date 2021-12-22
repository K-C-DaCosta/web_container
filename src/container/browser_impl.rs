use super::*;

use js_sys::{ArrayBuffer, Uint8Array};
use wasm_bindgen::{prelude::*, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, Request, RequestInit, RequestMode, Response};

use std::{io::Cursor, path::Path};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace=console,js_name=log)]
    fn log_js(val: JsValue);

    #[wasm_bindgen(js_namespace=console,js_name=log)]
    fn log_u32(val: u32);
}

pub struct WebContainer {
    header: Option<Header>,
}

impl WebContainer {
    pub fn new() -> Self {
        Self { header: None }
    }
    pub async fn open<P>(&mut self, path: P) -> ContainerError<()>
    where
        P: AsRef<Path>,
    {
        let mut buffer = [0u8; 8];

        // the first 24 bytes contain crucial info about size of the pile
        let header_metadata = Self::fetch_bytes(&path, 0, 24).await?;

        let mut metadata_slice = header_metadata.as_slice();

        metadata_slice.read_exact(&mut buffer).unwrap();
        let _num_entries = u64::from_le_bytes(buffer);

        metadata_slice.read_exact(&mut buffer).unwrap();
        let header_size = u64::from_le_bytes(buffer);

        metadata_slice.read_exact(&mut buffer).unwrap();
        let _total_size = u64::from_le_bytes(buffer);

        // log(&format!(
        //     "num entries {}, header_size {} , total_size {}",
        //     num_entries, header_size, total_size
        // ));

        let mut full_header = Cursor::new(Self::fetch_bytes(&path, 0, header_size as i64).await?);
        let header = Header::load(&mut full_header).unwrap();
        self.header = Some(header);

        // log(&format!("header\n {:?}", header));

        Ok(())
    }

    pub async fn get_file<P, Memory>(&mut self, path: P, mut out: Memory) -> Option<u64>
    where
        P: AsRef<Path>,
        Memory: Write,
    {
        let path_str = path.as_ref().to_str();
        let queried_entry = self
            .header
            .as_ref()
            .map(|hdr| {
                hdr.entries.iter().find(|entry| {
                    entry
                        .file_path_as_str()
                        .zip(path_str)
                        .map(|(e_pth, p)| e_pth == p)
                        .unwrap_or(false)
                })
            })
            .flatten();

        if let Some(entry) = queried_entry {
            let lbound = entry.offset() as i64;
            let ubound = entry.file_size() as i64;
            let mut file_bytes = Cursor::new(Self::fetch_bytes(&path, lbound, ubound).await.ok()?);
            io::copy(&mut file_bytes, &mut out).ok()?;
            Some(ubound as u64)
        } else {
            None
        }
    }

    async fn fetch_bytes<P: AsRef<Path>>(
        path: P,
        lbound: i64,
        ubound: i64,
    ) -> ContainerError<Vec<u8>> {
        let window = web_sys::window().ok_or(ErrorKind::GenericTextError("window failed"))?;
        let mut opts = RequestInit::new();
        let path = path.as_ref();

        opts.method("GET")
            .mode(RequestMode::Cors)
            .headers(&Self::range_headers(lbound, ubound).unwrap());

        let req = Request::new_with_str_and_init(path.to_str().unwrap(), &opts)
            .map_err(|_| ErrorKind::GenericTextError("fetch failed"))?;

        let response = JsFuture::from(window.fetch_with_request(&req))
            .await
            .map_err(|_| ErrorKind::GenericTextError("fetch failed"))?
            .dyn_into::<Response>()
            .map_err(|_| ErrorKind::GenericTextError("response failed to cast"))?;

        let blob = JsFuture::from(response.blob().unwrap())
            .await
            .map_err(|_| ErrorKind::GenericTextError("blob failed to resolve"))?
            .dyn_into::<Blob>()
            .map_err(|_| ErrorKind::GenericTextError("blob failed to cast"))?;

        let array_buffer = JsFuture::from(blob.array_buffer())
            .await
            .unwrap()
            .dyn_into::<ArrayBuffer>()
            .unwrap();

        let byte_array = Uint8Array::new(&array_buffer);
        Ok(byte_array.to_vec())
    }

    fn range_headers(lbound: i64, ubound: i64) -> Option<JsValue> {
        let json = format!(" {{ \"Range\" : \"bytes={}-{}\" }}   ", lbound, ubound - 1);
        let r = js_sys::JSON::parse(&json).ok().map(|a| {
            log_js(a.clone());
            a
        });
        r
    }
}
