use crate::{ZipContainerError, ZipContainerResult, BufFile, ok_or_err, io_err, function_path, invalid_data_err, reqwest_err};
use std::{fs::{File as StdFile}, path::{Path as StdPath}, io::{Read as StIoRead}};

pub trait FileLoader {
    fn load(&self, path_or_url: &str) -> impl std::future::Future<Output = ZipContainerResult<Vec<u8>>> + Send;
}
pub struct LocalFileLoader;

#[cfg(target_arch = "wasm32")]
impl FileLoader for UnifiedFileLoader {
    async fn load(&self, path_or_url: &str) -> ZipContainerResult<Vec<u8>> {
        // In WASM, all file operations need to be performed via HTTP requests
        let response = reqwest_err!(reqwest::get(path_or_url).await)?;
        let bytes = reqwest_err!(response.bytes().await)?;
        Ok(bytes.to_vec())
    }
}
#[cfg(not(target_arch = "wasm32"))]
impl FileLoader for LocalFileLoader {
    async fn load(&self, path: &str) -> ZipContainerResult<Vec<u8>> {
        let mut file = io_err!(StdFile::open(StdPath::new(path)))?;
        let mut buffer = Vec::new();
        io_err!(file.read_to_end(&mut buffer))?;
        Ok(buffer)
    }
}
pub struct UnifiedFileLoader;
#[cfg(not(target_arch = "wasm32"))]
impl FileLoader for UnifiedFileLoader {
    async fn load(&self, path_or_url: &str) -> ZipContainerResult<Vec<u8>> {
        if path_or_url.starts_with("http://") || path_or_url.starts_with("https://") {
            let response = reqwest_err!(reqwest::get(path_or_url).await)?;
            let bytes = reqwest_err!(response.bytes().await)?;
            Ok(bytes.to_vec())
        } else {
            let mut file = io_err!(StdFile::open(StdPath::new(path_or_url)))?;
            let mut buffer = Vec::new();
            io_err!(file.read_to_end(&mut buffer))?;
            Ok(buffer)
        }
    }
}
pub trait ZipContainerTrait {
    /// Returns a reference to the ZIP data buffer.
    fn zip_data(&self) -> ZipContainerResult<&[u8]> {
        ok_or_err!(None::<&[u8]>, "zip_data is not set")
    }

    /// Returns the path to the definition file within the ZIP.
    fn definition_path(&self) -> ZipContainerResult<&str> {
        ok_or_err!(None::<&str>, "definition_path is not set")
    }

    /// Sets the files after processing.
    fn set_files(&mut self, files: Vec<BufFile>);

    /// Loads a file from the ZIP data.
    fn load_file_from_zip(&self, file_path: &str) -> impl std::future::Future<Output = ZipContainerResult<Vec<u8>>> + Send where Self: Sync {async {
        let zip_data = self.zip_data()?;
        let reader = std::io::Cursor::new(zip_data);
        let mut zip = io_err!(zip::ZipArchive::new(reader))?;
        let mut file = io_err!(zip.by_name(file_path))?;
        let mut buffer = Vec::new();
        io_err!(file.read_to_end(&mut buffer))?;
        Ok(buffer)
    } }

    /// Loads a file as a string from the ZIP data.
    fn load_file_str(&self, file_path: &str) -> impl std::future::Future<Output = ZipContainerResult<String>> + Send where Self: Sync {async {
        let buffer = self.load_file_from_zip(file_path).await?;
        let content = invalid_data_err!(String::from_utf8(buffer))?;
        Ok(content)
    } }

    /// Loads the definition file as a string.
    fn load_definition_file_str(&self) -> impl std::future::Future<Output = ZipContainerResult<String>> + Send where Self: Sync {async {
        let definition_path = self.definition_path()?;
        self.load_file_str(definition_path).await
    } }

    /// Loads a file either from the ZIP data or from a URL.
    fn load_file(&self, file_path_or_url: &str) -> impl std::future::Future<Output = ZipContainerResult<Vec<u8>>> + Send where Self: Sync {async move {
        // Attempt to load from ZIP data
        if let Ok(data) = self.load_file_from_zip(file_path_or_url).await {
            return Ok(data);
        }

        // If not found in ZIP, check if it's a URL
        if file_path_or_url.starts_with("http://") || file_path_or_url.starts_with("https://") {
            // Fetch from URL
            let response = reqwest_err!(reqwest::get(file_path_or_url).await)?;
            let bytes = reqwest_err!(response.bytes().await)?;
            return Ok(bytes.to_vec());
        }

        // File not found
        Err(ZipContainerError::MissingValue {
            module_path: function_path!(),
            message: format!("File '{}' not found in ZIP data or accessible via URL", file_path_or_url),
        })
    } }

    /// Processes files referenced in the definition.
    fn process_files(&mut self) -> impl std::future::Future<Output = ZipContainerResult<()>> + Send where Self: Sync, Self: Send {async {
        let file_paths = self.extract_file_paths_from_definition().await?;
        let mut files = Vec::new();
        for path in file_paths {
            let content = self.load_file(&path).await?;
            files.push(BufFile {
                name: Some(path.clone()),
                content: Some(content),
                path: Some(path),
                ..Default::default()
            });
        }
        self.set_files(files);
        Ok(())
    } }

    /// Extracts file paths from the definition content.
    fn extract_file_paths_from_definition(&self) -> impl std::future::Future<Output = ZipContainerResult<Vec<String>>> + Send {async {
        // let definition_content = self.load_definition_file_str().await?;
        // Parse the definition_content to extract file paths
        // This needs to be implemented as per your definition file format
        Ok(vec![]) // Placeholder
    } }

    /// get all files from the zip

    fn get_zip_files(&self) -> ZipContainerResult<Vec<BufFile>> {
        let zip_data = self.zip_data()?;
        let reader = std::io::Cursor::new(zip_data);
        let mut zip = io_err!(zip::ZipArchive::new(reader))?;
        let mut files = Vec::new();

        for i in 0..zip.len() {
            let mut file = io_err!(zip.by_index(i))?;
            let mut buffer = Vec::new();
            io_err!(file.read_to_end(&mut buffer))?;
            files.push(BufFile {
                name: Some(file.name().to_string()),
                content: Some(buffer),
                path: Some(file.name().to_string()),
                size: Some(file.size()),
                ..Default::default()
            });
        }
        Ok(files)
    }
}

