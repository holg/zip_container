use crate::UnifiedFileLoader;
use crate::FileLoader;
use pyo3::prelude::*;
use crate::{ZipContainer as ZipContainerRust, ZipContainerResult, ZipContainerTrait};
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

/// A lazily initialized Tokio runtime.
static RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
});

/// A wrapper around the Rust `ZipContainer` struct for Python
#[pyclass]
struct ZipContainer {
    zip_container: ZipContainerRust,
}

#[pymethods]
impl ZipContainer {
    /// Load a zip file into the ZipContainer structure
    #[new]
    fn new(path: &str, definition_path: Option<String>) -> PyResult<Self> {
        // Create a loader
        let loader: Box<dyn FileLoader + Send + Sync> = Box::new(UnifiedFileLoader);

        // Use the runtime to block on the async function
        let zip_container_result = RUNTIME.block_on(async {
            ZipContainerRust::new(loader, Some(path), definition_path.as_deref()).await
        });

        match zip_container_result {
            Ok(zip_container) => Ok(Self { zip_container }),
            Err(e) => Err(pyo3::exceptions::PyException::new_err(format!("{}", e))),
        }
    }

    // Implement other methods as needed
}

/// Python module definition
#[pymodule]
fn zip_container(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ZipContainer>()?; // Add the ZipContainer class to the module
    Ok(())
}
