// lib.rs lib of zip_container
#[cfg(test)]
mod tests;

// use crate::{ok_or_err, io_err};
use std::path::Path;
pub use container_error::{ZipContainerError};
pub use zip_container_trait::{ZipContainerTrait, UnifiedFileLoader};
// use std::path::Path;
use crate::zip_container_trait::FileLoader;

pub type ZipContainerResult<T> = Result<T, ZipContainerError>;

pub trait Logger {
    fn log(&self, message: &str);
}
pub trait AsyncLogger {
    fn log(&self, message: &str) -> impl std::future::Future<Output = ()> + Send;
}

#[derive(Clone, Debug)]
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
    pub fn new(zip_path: String, definition_path: Option<String>) -> Self {
    let loader = UnifiedFileLoader;
    let zip_data = Some(futures::executor::block_on(loader.load(&zip_path)).unwrap());
    let zip_data = Some(zip_data.unwrap_or_else(|| Vec::new()));
    ZipContainer {
        zip_data,
        definition_path: definition_path.clone(),
        definition_content: definition_path.and_then(|path| {
            match Path::new(&path.to_lowercase()).extension().and_then(|ext| ext.to_str()) {
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


#[derive(Clone, Debug, Default)]
pub struct BufFile{
    pub name: Option<String>,
    pub content: Option<Vec<u8>>,
    pub file_id: Option<String>,
    pub content_type: Option<String>,
    pub path: Option<String>,
    pub size: Option<u64>,
}
// #[derive(Clone, Debug, Default)]
// pub struct FileBufZipcontainer<'a>{
//     pub files: Vec<BufFile>,
//     pub zip_container: ZipContainer,
// }

pub fn process_zip_container(path: &str) -> ZipContainerResult<()> {
    println!("Processing ZIP container at: {}", path);
    Ok(())
}

// Python bindings (only compiled when the 'python' feature is enabled)
#[cfg(feature = "python")]
pub mod python_bindings;
pub mod container_error;
pub mod zip_container_trait;
// mod ::python_tests;
