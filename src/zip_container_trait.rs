// src/zip_container_trait.rs

use crate::{
    function_path, io_err, reqwest_err, BufFile,ZipContainerError,
    ZipContainerResult,
};
use std::future::Future;
use std::io::Read as StdIoRead;
use std::pin::Pin;
use std::string::FromUtf8Error;

#[cfg(not(target_arch = "wasm32"))]
type ZipContainerFuture<T> = Pin<Box<dyn Future<Output = ZipContainerResult<T>> + Send>>;

#[cfg(target_arch = "wasm32")]
type ZipContainerFuture<T> = Pin<Box<dyn Future<Output = ZipContainerResult<T>>>>;

pub trait AsyncFileLoader {
    fn load_async(&self, path_or_url: &str) -> ZipContainerFuture<Vec<u8>>;
}

// Synchronous FileLoader trait
pub trait FileLoader {
    fn load(&self, path_or_url: &str) -> ZipContainerResult<Vec<u8>>;
}

// UnifiedFileLoader available on all targets
pub struct UnifiedFileLoader;

impl FileLoader for UnifiedFileLoader {
    fn load(&self, path_or_url: &str) -> ZipContainerResult<Vec<u8>> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if path_or_url.starts_with("http://") || path_or_url.starts_with("https://") {
                // Synchronous HTTP requests using reqwest::blocking
                let response = reqwest_err!(reqwest::blocking::get(path_or_url))?;
                let bytes = reqwest_err!(response.bytes())?;
                Ok(bytes.to_vec())
            } else {
                // Read from the local filesystem
                let mut file = io_err!(StdFile::open(StdPath::new(path_or_url)))?;
                let mut buffer = Vec::new();
                io_err!(file.read_to_end(&mut buffer))?;
                Ok(buffer)
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            // Return an error indicating that synchronous loading isn't supported in WASM
            Err(ZipContainerError::UnsupportedOperation {
                module_path: function_path!(),
                message: "Synchronous file loading is not supported in WASM".to_string(),
            })
        }
    }
}

// Unified asynchronous file loader
pub struct UnifiedAsyncFileLoader;

impl AsyncFileLoader for UnifiedAsyncFileLoader {
    fn load_async(&self, path_or_url: &str) -> ZipContainerFuture<Vec<u8>> {
        let path_or_url = path_or_url.to_string(); // Own the data
        Box::pin(async move {
            if path_or_url.starts_with("http://") || path_or_url.starts_with("https://") {
                // Load from URL using asynchronous HTTP client
                let response = reqwest_err!(reqwest::get(&path_or_url).await)?;
                let bytes = reqwest_err!(response.bytes().await)?;
                Ok(bytes.to_vec())
            } else {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    // Load from local filesystem asynchronously using tokio::fs
                    use tokio::fs::File;
                    use tokio::io::AsyncReadExt;

                    let mut file = io_err!(File::open(StdPath::new(&path_or_url)).await)?;
                    let mut buffer = Vec::new();
                    io_err!(file.read_to_end(&mut buffer).await)?;
                    Ok(buffer)
                }
                #[cfg(target_arch = "wasm32")]
                {
                    // Return an error since local file access is not available in the browser
                    Err(ZipContainerError::UnsupportedOperation {
                        module_path: function_path!(),
                        message: "Local file access is not supported in this environment"
                            .to_string(),
                    })
                }
            }
        })
    }
}

// Implementing the From trait to convert FromUtf8Error to ZipContainerError
impl From<FromUtf8Error> for ZipContainerError {
    fn from(err: FromUtf8Error) -> ZipContainerError {
        ZipContainerError::Utf8Error(err)
    }
}

// The main trait defining synchronous and asynchronous methods
pub trait ZipContainerTrait: Clone + Send + Sync + 'static {
    // Synchronous methods

    /// Returns a reference to the ZIP data buffer.
    fn zip_data(&self) -> ZipContainerResult<&[u8]>;

    /// Returns the path to the definition file within the ZIP.
    fn definition_path(&self) -> ZipContainerResult<&str>;

    /// Returns a list of file names in the ZIP archive.
    fn get_file_names(&self) -> ZipContainerResult<Vec<String>> {
        let zip_data = self.zip_data()?;
        let reader = std::io::Cursor::new(zip_data);
        let mut zip = io_err!(zip::ZipArchive::new(reader))?;
        let mut file_names = Vec::new();

        for i in 0..zip.len() {
            let file = io_err!(zip.by_index(i))?;
            file_names.push(file.name().to_string());
        }

        Ok(file_names)
    }

    /// Returns a list of file names in the ZIP archive.
    fn get_zip_files(&self) -> ZipContainerResult<Vec<BufFile>> {
        let zip_data = self.zip_data()?;
        let reader = std::io::Cursor::new(zip_data);
        let mut zip = io_err!(zip::ZipArchive::new(reader))?;
        let mut zip_files:Vec<BufFile> = Vec::new();

        for i in 0..zip.len() {
            let file = io_err!(zip.by_index(i))?;
            zip_files.push({
                let mut buf_file = BufFile::default();
                buf_file.name = Some(file.name().to_string());
                buf_file.size = Some(file.size());
                buf_file.path = Some(file.mangled_name().display().to_string());
                buf_file.file_id = Some(i.to_string());
                buf_file
            });
        }

        Ok(zip_files)
    }


    /// Sets the files after processing.
    fn set_files(&mut self, files: Vec<BufFile>);

    /// Loads a file from the ZIP data synchronously.
    fn load_file_from_zip(&self, file_path: &str) -> ZipContainerResult<Vec<u8>> {
        let zip_data = self.zip_data()?;
        let reader = std::io::Cursor::new(zip_data);
        let mut zip = io_err!(zip::ZipArchive::new(reader))?;
        let mut file = io_err!(zip.by_name(file_path))?;
        let mut buffer = Vec::new();
        io_err!(file.read_to_end(&mut buffer))?;
        Ok(buffer)
    }

    /// Loads a file either from the ZIP data or from a URL synchronously.
    #[cfg(not(target_arch = "wasm32"))]
    fn load_file(&self, file_path_or_url: &str) -> ZipContainerResult<Vec<u8>> {
        // Attempt to load from ZIP data
        if let Ok(data) = self.load_file_from_zip(file_path_or_url) {
            return Ok(data);
        }

        // Load using the unified file loader
        let loader = UnifiedFileLoader;
        loader.load(file_path_or_url)
    }

    #[cfg(target_arch = "wasm32")]
    fn load_file(&self, _file_path_or_url: &str) -> ZipContainerResult<Vec<u8>> {
        // Return an error since synchronous file loading is not supported in WASM
        Err(ZipContainerError::UnsupportedOperation {
            module_path: function_path!(),
            message: "Synchronous file loading is not supported in WASM".to_string(),
        })
    }

    // Asynchronous methods

    /// Loads a file from the ZIP data asynchronously.
    fn load_file_from_zip_async(&self, file_path: &str) -> ZipContainerFuture<Vec<u8>> {
        let zip_data = match self.zip_data() {
            Ok(data) => data.to_vec(),
            Err(e) => return Box::pin(async move { Err(e) }),
        };
        let file_path = file_path.to_string(); // Own the data
        Box::pin(async move {
            let reader = std::io::Cursor::new(zip_data);
            let mut zip = io_err!(zip::ZipArchive::new(reader))?;
            let mut file = io_err!(zip.by_name(&file_path))?;
            let mut buffer = Vec::new();
            io_err!(file.read_to_end(&mut buffer))?;
            Ok(buffer)
        })
    }

    /// Loads a file either from the ZIP data or from a URL asynchronously.
    fn load_file_async(&self, file_path_or_url: &str) -> ZipContainerFuture<Vec<u8>> {
        let self_clone = self.clone();
        let path_or_url = file_path_or_url.to_string(); // Own the data
        Box::pin(async move {
            // Attempt to load from ZIP data
            if let Ok(data) = self_clone.load_file_from_zip_async(&path_or_url).await {
                return Ok(data);
            }

            // Load using the unified async file loader
            let loader = UnifiedAsyncFileLoader;
            loader.load_async(&path_or_url).await
        })
    }
    fn load_definition_file_str(&self) -> ZipContainerResult<String> {
        let definition_path = self.definition_path()?;
        let definition_content = self.load_file(definition_path)?;
        let definition_content = String::from_utf8(definition_content)?;
        Ok(definition_content)
    }
    fn process_files(&mut self) -> ZipContainerResult<()> {
        let zip_files = self.get_zip_files()?;
        self.set_files(zip_files);
        Ok(())
    }

    // Additional methods can be added here, following the same pattern.
}