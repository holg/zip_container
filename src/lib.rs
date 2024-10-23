// src/lib.rs lib of zip_container
pub mod container_error;
pub mod zip_container_trait;
pub use container_error::{ZipContainerError};
pub use zip_container_trait::{ZipContainerTrait, UnifiedFileLoader, FileLoader};
pub type ZipContainerResult<T> = Result<T, ZipContainerError>;
use serde::{Serialize, Deserialize};
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path as StdPath;
pub trait Logger {
    fn log(&self, message: &str);
}
pub trait AsyncLogger {
    fn log(&self, message: &str) -> impl std::future::Future<Output = ()> + Send;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Definition {
    XML(String),
    JSON(String),
    YAML(String),
    TOML(String),
}
impl Default for Definition {
    fn default() -> Self {
        Definition::XML(String::new())
    }
}

impl AsRef<str> for Definition {
    fn as_ref(&self) -> &str {
        match self {
            Definition::XML(ref s) |
            Definition::JSON(ref s) |
            Definition::YAML(ref s) |
            Definition::TOML(ref s) => s,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ZipContainer {
    pub zip_data: Option<Vec<u8>>,
    pub definition_path: Option<String>,
    pub definition_content: Option<Definition>,
    pub files: Option<Vec<BufFile>>,
    pub zip_path: Option<String>,
}
impl ZipContainer {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_zip_data(zip_path: &str) -> Option<Vec<u8>> {
        let loader = UnifiedFileLoader;
        match loader.load(zip_path) {
            Ok(data) => Some(data),
            Err(e) => {
                // Log the error
                eprintln!("Failed to load ZIP data: {:?}", e);
                None
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(zip_path: String, definition_path: Option<String>) -> Self {
        ZipContainer {
            zip_data: ZipContainer::load_zip_data(&zip_path, /* Option<&str> */),
            definition_path: definition_path.clone(),
            definition_content: definition_path.and_then(|path| {
                match StdPath::new(&path.to_lowercase()).extension().and_then(|ext| ext.to_str()) {
                    Some("xml") => Some(Definition::XML(String::new())),
                    Some("json") => Some(Definition::JSON(String::new())),
                    Some("yaml") => Some(Definition::YAML(String::new())),
                    Some("toml") => Some(Definition::TOML(String::new())),
                    _ => None,
                }
            }),
            files: None,
            zip_path: Some(zip_path),
        }
    }
    pub async fn read_file_async(&self, file_name: &str) -> ZipContainerResult<Vec<u8>> {
        self.load_file_async(file_name).await
    }

}

impl ZipContainerTrait for ZipContainer {
    fn zip_data(&self) -> ZipContainerResult<&[u8]> {
        ok_or_err!(self.zip_data.as_deref(), "zip_data is not set")
    }

    fn definition_path(&self) -> ZipContainerResult<&str> {
        ok_or_err!(self.definition_path.as_deref(), "definition_path is not set")
    }

    fn set_files(&mut self, files: Vec<BufFile>) {
        self.files = Some(files);
    }

    // No need to implement other methods; default implementations are used
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BufFile{
    pub name: Option<String>,
    pub content: Option<Vec<u8>>,
    pub file_id: Option<String>,
    pub content_type: Option<String>,
    pub path: Option<String>,
    pub size: Option<u64>,
}


// Python bindings (only compiled when the 'python' feature is enabled)
#[cfg(feature = "python")]
pub mod python_bindings;
#[cfg(target_arch = "wasm32")]
pub mod wasm_bindings;
mod tests;
// mod ::python_tests;
