// src/wasm_bindings.rs

use crate::zip_container_trait::ZipContainerTrait;
use crate::ZipContainer;
use js_sys::{Function, Promise, Reflect, Uint8Array};
use serde_wasm_bindgen;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{future_to_promise, JsFuture};

#[wasm_bindgen]
pub struct WasmZipContainer {
    inner: Rc<ZipContainer>,
}

#[wasm_bindgen]
impl WasmZipContainer {
    /// Constructor that accepts raw ZIP data as a Uint8Array
    #[wasm_bindgen(constructor)]
    pub fn new(
        zip_data: Uint8Array,
        definition_path: Option<String>,
    ) -> Result<WasmZipContainer, JsValue> {
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
            // Get the global object (works in both browser and Node.js)
            let global = js_sys::global();

            // Get the 'fetch' function from the global object
            let fetch_fn = Reflect::get(&global, &JsValue::from_str("fetch"))
                .map_err(|_| JsValue::from_str("Failed to get 'fetch' function"))?;

            // Ensure that 'fetch' is a function
            let fetch_fn = fetch_fn
                .dyn_into::<Function>()
                .map_err(|_| JsValue::from_str("'fetch' is not a function"))?;

            // Call 'fetch' with the URL
            let fetch_promise_value = fetch_fn
                .call1(&global, &JsValue::from_str(&url))
                .map_err(|_| JsValue::from_str("Failed to call 'fetch' function"))?;

            // Convert fetch_promise_value to js_sys::Promise
            let fetch_promise = fetch_promise_value
                .dyn_into::<js_sys::Promise>()
                .map_err(|_| JsValue::from_str("Failed to cast fetch result to Promise"))?;

            let resp_value = JsFuture::from(fetch_promise)
                .await
                .map_err(|e| JsValue::from_str(&format!("Fetch error: {:?}", e)))?;

            let resp = resp_value
                .dyn_into::<web_sys::Response>()
                .map_err(|_| JsValue::from_str("Failed to cast to Response"))?;

            if !resp.ok() {
                return Err(JsValue::from_str(&format!(
                    "Network response was not ok: {}",
                    resp.status()
                )));
            }

            // Get the array buffer promise
            let array_buffer_promise_value = resp
                .array_buffer()
                .map_err(|_| JsValue::from_str("Failed to get array buffer"))?;

            // Convert to js_sys::Promise
            let array_buffer_promise = array_buffer_promise_value
                .dyn_into::<js_sys::Promise>()
                .map_err(|_| JsValue::from_str("Failed to cast array buffer result to Promise"))?;

            let array_buffer = JsFuture::from(array_buffer_promise)
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
    pub fn read_file(&self, file_name: &str) -> Promise {
        let container = self.inner.clone();
        let file_name = file_name.to_string();

        let fut = async move {
            container
                .read_file_async(&file_name)
                .await
                .map(|content| Uint8Array::from(&content[..]))
                .map(JsValue::from)
                .map_err(|e| JsValue::from_str(&format!("Error: {:?}", e)))
        };

        wasm_bindgen_futures::future_to_promise(fut)
    }

    // Add additional methods as needed
}
