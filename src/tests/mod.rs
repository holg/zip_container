// src/tests.rs
#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
    use crate::{BufFile, Definition, ZipContainer, ZipContainerTrait};
    // use super::*;
    // use std::path::Path;

    #[test]
    fn test_zip_data_not_set() {
        let zip_container = ZipContainer::default();
        let result = zip_container.zip_data();
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(
                e.to_string().ends_with("zip_data is not set"),
                true)
        }
    }

    #[test]
    fn test_definition_path_not_set() {
        let zip_container = ZipContainer::default();
        let result = zip_container.definition_path();
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.to_string().ends_with("definition_path is not set"), true);
        }
    }

    #[test]
    fn test_initialize_zip_container_with_valid_xml_definition_path() {
        let zip_path = String::from("test.zip");
        let definition_path = Some(String::from("definition.xml"));
        let zip_container = ZipContainer::new(zip_path.clone(), definition_path.clone());

        assert_eq!(zip_container.zip_path, Some(zip_path));
        assert_eq!(zip_container.definition_path, definition_path);
        assert!(matches!(zip_container.definition_content, Some(Definition::XML(_))));
    }


    #[test]
    fn test_initialize_zip_container_with_default_values() {
        let zip_container = ZipContainer::default();

        assert!(zip_container.zip_data.is_none());
        assert!(zip_container.definition_path.is_none());
        assert!(zip_container.definition_content.is_none());
        assert!(zip_container.files.is_none());
        assert!(zip_container.zip_path.is_none());
    }

    #[test]
    fn test_initialize_zip_container_with_valid_json_definition_path() {
        let zip_path = String::from("test.zip");
        let definition_path = Some(String::from("definition.json"));
        let zip_container = ZipContainer::new(zip_path.clone(), definition_path.clone());

        assert_eq!(zip_container.zip_path, Some(zip_path));
        assert_eq!(zip_container.definition_path, definition_path);
        assert!(matches!(zip_container.definition_content, Some(Definition::JSON(_))));
    }
    #[test]
    fn test_initialize_zip_container_with_unsupported_definition_path() {
        let zip_path = String::from("test.zip");
        let definition_path = Some(String::from("definition.unsupported"));
        let zip_container = ZipContainer::new(zip_path.clone(), definition_path.clone());

        assert_eq!(zip_container.zip_path, Some(zip_path));
        assert_eq!(zip_container.definition_path, definition_path);
        assert!(zip_container.definition_content.is_none());
    }

    #[test]
    fn test_set_files_in_zip_container() {
        let mut zip_container = ZipContainer::default();
        let files = vec![
            BufFile {
                name: Some(String::from("file1.txt")),
                content: Some(vec![1, 2, 3]),
                file_id: Some(String::from("id1")),
                content_type: Some(String::from("text/plain")),
                path: Some(String::from("path/to/file1.txt")),
                size: Some(3),
            },
            BufFile {
                name: Some(String::from("file2.txt")),
                content: Some(vec![4, 5, 6]),
                file_id: Some(String::from("id2")),
                content_type: Some(String::from("text/plain")),
                path: Some(String::from("path/to/file2.txt")),
                size: Some(3),
            },
        ];

        zip_container.set_files(files.clone());

        assert_eq!(zip_container.files, Some(files));
    }

    #[test]
    fn test_process_zip_container_with_valid_path() {
        let result = ZipContainer::new(
            "https://raw.githubusercontent.com/holg/gldf-rs/refs/heads/master/tests/data/test.gldf".to_string(),
            Some("product.xml".to_string()),
        );
        assert_eq!(result.definition_path.unwrap(), "product.xml");
        // Assuming ZipContainer::new returns a Result, you need to handle it accordingly.
        // If it doesn't, you need to adjust the test to match the actual return type.
        // assert!(result.is_ok());
    }

    #[test]
    fn test_definition_as_ref() {
        let xml_definition = Definition::XML(String::from("xml content"));
        let json_definition = Definition::JSON(String::from("json content"));
        let yaml_definition = Definition::YAML(String::from("yaml content"));
        let toml_definition = Definition::TOML(String::from("toml content"));

        assert_eq!(xml_definition.as_ref(), "xml content");
        assert_eq!(json_definition.as_ref(), "json content");
        assert_eq!(yaml_definition.as_ref(), "yaml content");
        assert_eq!(toml_definition.as_ref(), "toml content");
    }

    #[cfg(target_arch = "wasm32")]
    #[tokio::test]
    async fn test_async_initialize_zip_container_on_wasm32() {
        let zip_path = String::from("test.zip");
        let definition_path = Some(String::from("definition.json"));
        let zip_container = ZipContainer::new(zip_path.clone(), definition_path.clone()).await;

        assert_eq!(zip_container.zip_path, Some(zip_path));
        assert_eq!(zip_container.definition_path, definition_path);
        assert!(matches!(zip_container.definition_content, Some(Definition::JSON(_))));
    }
}

// src/tests.rs

#[cfg(test)]
#[cfg(target_arch = "wasm32")]
mod tests {
    use crate::wasm_bindings::WasmZipContainer;
    // use crate::ZipContainerTrait;
    use js_sys::{Promise, Uint8Array, Reflect, Function};
    use wasm_bindgen::{JsCast, JsValue};

    use wasm_bindgen_futures::JsFuture;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser );
    #[wasm_bindgen_test]
    async fn test_get_file_names() {
        // Call from_url, which returns a Promise
        let container_promise = WasmZipContainer::from_url(
            "https://raw.githubusercontent.com/holg/gldf-rs/refs/heads/master/tests/data/test.gldf".to_string(),
            Some("product.xml".to_string()),
        );

        // Await the Promise to get the JsValue representing the WasmZipContainer
        let container_js_value = JsFuture::from(container_promise).await.unwrap();

        // Get the `get_file_names` method from the JavaScript object
        let get_file_names = Reflect::get(&container_js_value, &JsValue::from_str("get_file_names"))
            .unwrap()
            .dyn_into::<Function>()
            .unwrap();

        // Call the `get_file_names` method
        let file_names_js_value = get_file_names.call0(&container_js_value).unwrap();

        // Convert JsValue to Vec<String>
        let file_names: Vec<String> = serde_wasm_bindgen::from_value(file_names_js_value).unwrap();

        // Perform your assertions
        assert!(file_names.contains(&"product.xml".to_string()));
        assert!(file_names.contains(&"ldc/diffuse.ldt".to_string()));
    }
    #[wasm_bindgen_test]
    async fn test_read_file() {
        // Similar setup as before
        let zip_data = include_bytes!("../../test_data/test.gldf").to_vec();
        let zip_data_js = Uint8Array::from(&zip_data[..]);
        let container = WasmZipContainer::new(zip_data_js, None).unwrap();

        // Read a specific file
        let promise = container.read_file("product.xml");
        let js_value = JsFuture::from(Promise::from(promise))
            .await
            .unwrap();

        // Convert JsValue (Uint8Array) to Vec<u8>
        let uint8_array = Uint8Array::new(&js_value);
        let mut content = vec![0; uint8_array.length() as usize];
        uint8_array.copy_to(&mut content);

        // Convert content to string
        let content_str = String::from_utf8(content).unwrap();

        // Perform assertions on the content
        assert!(content_str.contains("<Product"));
    }
}