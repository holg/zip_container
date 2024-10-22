#[cfg(test)]
mod tests {
    use crate::{ZipContainer, ZipContainerTrait, BufFile, Definition};
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
