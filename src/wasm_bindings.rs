// src/wasm_bindings.rs

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use serde_wasm_bindgen;
use js_sys::{Promise, Uint8Array};
use crate::{ZipContainer};
use crate::zip_container_trait::ZipContainerTrait;
use std::rc::Rc;

#[wasm_bindgen]
pub struct WasmZipContainer {
    inner: Rc<ZipContainer>,
}

#[wasm_bindgen]
impl WasmZipContainer {
    /// Constructor that accepts raw ZIP data as a Uint8Array
    #[wasm_bindgen(constructor)]
    pub fn new(zip_data: Uint8Array, definition_path: Option<String>) -> Result<WasmZipContainer, JsValue> {
        // Convert Uint8Array to Vec<u8>
        let mut data = vec![0; zip_data.length() as usize];
        zip_data.copy_to(&mut data);

        // Create ZipContainer instance
        let zip_container = ZipContainer {
            zip_data: Some(data),
            definition_path: definition_path.clone(),
            definition_content: None,
            files: None,
            zip_path: None,
        };

        Ok(WasmZipContainer {
            inner: Rc::new(zip_container),
        })
    }

    /// Asynchronous constructor that fetches ZIP data from a URL
    #[wasm_bindgen]
    pub fn from_url(url: String, definition_path: Option<String>) -> Promise {
        let definition_path_clone = definition_path.clone();

        let fut = async move {
            // Fetch the ZIP data from the URL
            let window = web_sys::window().ok_or_else(|| JsValue::from_str("No global `window` exists"))?;
            let resp_value = JsFuture::from(window.fetch_with_str(&url))
                .await
                .map_err(|e| JsValue::from_str(&format!("Fetch error: {:?}", e)))?;
            let resp: web_sys::Response = resp_value.dyn_into().map_err(|_| JsValue::from_str("Failed to cast to Response"))?;
            if !resp.ok() {
                return Err(JsValue::from_str(&format!("Network response was not ok: {}", resp.status())));
            }
            let array_buffer = JsFuture::from(resp.array_buffer()?)
                .await
                .map_err(|e| JsValue::from_str(&format!("ArrayBuffer error: {:?}", e)))?;
            let uint8_array = Uint8Array::new(&array_buffer);

            // Convert Uint8Array to Vec<u8>
            let mut data = vec![0; uint8_array.length() as usize];
            uint8_array.copy_to(&mut data);

            // Create ZipContainer instance
            let zip_container = ZipContainer {
                zip_data: Some(data),
                definition_path: definition_path_clone,
                definition_content: None,
                files: None,
                zip_path: None,
            };

            let wasm_zip_container = WasmZipContainer {
                inner: Rc::new(zip_container),
            };

            Ok(JsValue::from(wasm_zip_container))
        };

        future_to_promise(fut)
    }

    /// Get list of file names in the ZIP archive
    #[wasm_bindgen]
    pub fn get_file_names(&self) -> Result<JsValue, JsValue> {
        self.inner
            .get_file_names()
            .map(|names| serde_wasm_bindgen::to_value(&names).unwrap())
            .map_err(|e| JsValue::from_str(&format!("Error: {:?}", e)))
    }

    /// Read a specific file's content from the ZIP archive
    #[wasm_bindgen]
    pub fn read_file(&self, file_name: &str) -> Result<Uint8Array, JsValue> {
        self.inner
            .load_file(file_name)
            .map(|content| Uint8Array::from(&content[..]))
            .map_err(|e| JsValue::from_str(&format!("Error: {:?}", e)))
    }

    // Add additional methods as needed
}