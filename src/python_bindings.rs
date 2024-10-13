// src/python_bindings.rs
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::exceptions::PyException;
use std::ops::Deref;
use crate::{ZipContainer as ZipContainerRust, BufFile, ZipContainerError };
use crate::zip_container_trait::ZipContainerTrait;
// Implementing necessary conversion from ZipContainerError to PyErr
impl From<ZipContainerError> for PyErr {
    /// Converts a `ZipContainerError` into a `PyErr`.
    ///
    /// This function facilitates the conversion of a `ZipContainerError` into a Python
    /// exception (`PyErr`) that can be raised in Python code. It formats the error
    /// message from the `ZipContainerError` to be used in the Python exception.
    ///
    /// # Parameters
    ///
    /// * `err` - The `ZipContainerError` instance that needs to be converted into a `PyErr`.
    ///
    /// # Returns
    ///
    /// A `PyErr` representing a Python exception with the formatted error message from
    /// the `ZipContainerError`.
    fn from(err: ZipContainerError) -> PyErr {
        PyException::new_err(format!("{:?}", err))
    }
}

//Implementing necessary conversion from BufFile to PyObject
impl<'a> FromPyObject<'a> for BufFile {
        /// Extracts a `BufFile` instance from a Python object.
    ///
    /// This function attempts to convert a given Python object into a `BufFile` by
    /// extracting its fields from a Python dictionary. Each field is extracted
    /// individually and converted to the corresponding Rust type.
    ///
    /// # Parameters
    ///
    /// * `obj` - A reference to a Python object (`PyAny`) that is expected to be a dictionary
    ///   containing the fields of a `BufFile`.
    ///
    /// # Returns
    ///
    /// A `PyResult` containing the `BufFile` instance if the extraction is successful,
    /// or an error if the conversion fails.
    fn extract(obj: &'a PyAny) -> PyResult<Self> {
        let dict = obj.downcast::<PyDict>()?;
        Ok(BufFile {
            name: dict.get_item("name").and_then(|x| x.extract().ok()),
            content: dict.get_item("content").and_then(|x| x.extract().ok()),
            file_id: dict.get_item("file_id").and_then(|x| x.extract().ok()),
            content_type: dict.get_item("content_type").and_then(|x| x.extract().ok()),
            path: dict.get_item("path").and_then(|x| x.extract().ok()),
            size: dict.get_item("size").and_then(|x| x.extract().ok()),
        })
    }
}

impl IntoPy<PyObject> for BufFile {
    /// Converts a `BufFile` instance into a Python dictionary (`PyObject`).
    ///
    /// This function takes ownership of the `BufFile` instance and transforms its fields
    /// into key-value pairs in a Python dictionary. Each field of the `BufFile` is checked
    /// for presence and, if available, is added to the dictionary with its corresponding key.
    ///
    /// # Parameters
    ///
    /// * `self` - The `BufFile` instance to be converted.
    /// * `py` - A Python interpreter token, which is required to create Python objects.
    ///
    /// # Returns
    ///
    /// A `PyObject` representing a Python dictionary containing the fields of the `BufFile`.
    /// Each field is added to the dictionary only if it is `Some`, otherwise it is omitted.
    fn into_py(self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        if let Some(name) = self.name {
            dict.set_item("name", name).unwrap();
        }
        if let Some(content) = self.content {
            dict.set_item("content", content).unwrap();
        }
        if let Some(file_id) = self.file_id {
            dict.set_item("file_id", file_id).unwrap();
        }
        if let Some(content_type) = self.content_type {
            dict.set_item("content_type", content_type).unwrap();
        }
        if let Some(path) = self.path {
            dict.set_item("path", path).unwrap();
        }
        if let Some(size) = self.size {
            dict.set_item("size", size).unwrap();
        }
        dict.into()
    }
}

/// A Python class that wraps the `ZipContainerRust` struct, providing Python bindings
/// for interacting with ZIP containers.
///
/// This class allows for the creation and manipulation of ZIP containers from Python code.
/// It provides methods to access and modify the ZIP data, definition path, and files within
/// the container.
///
/// # Fields
///
/// * `zip_container` - An instance of `ZipContainerRust` that this class wraps, providing
///   the core functionality for ZIP container operations.
#[pyclass]
struct ZipContainer {
    zip_container: ZipContainerRust,
}
impl Deref for ZipContainer {
    type Target = ZipContainerRust;

    /// Provides a reference to the underlying `ZipContainerRust` instance.
    ///
    /// This function allows the `ZipContainer` struct to be dereferenced to access
    /// the methods and fields of the `ZipContainerRust` it wraps.
    ///
    /// # Returns
    ///
    /// A reference to the `ZipContainerRust` instance contained within the `ZipContainer`.
    fn deref(&self) -> &Self::Target {
        &self.zip_container
    }
}

#[pymethods]
impl ZipContainer {
    /// Creates a new `ZipContainer` instance.
    ///
    /// This function initializes a `ZipContainer` with the specified path and an optional
    /// definition path. It constructs the underlying `ZipContainerRust` and wraps it
    /// within a Python-accessible class.
    ///
    /// # Parameters
    ///
    /// * `path` - A `String` representing the file path to the ZIP container.
    /// * `definition_path` - An `Option<String>` that specifies the path to the definition
    ///   file within the ZIP container. This parameter is optional and can be `None`.
    ///
    /// # Returns
    ///
    /// A `PyResult<Self>` which is an instance of `ZipContainer` if successful, or an error
    /// if the initialization fails.
    #[new]
    fn new(path: String, definition_path: Option<String>) -> PyResult<Self> {
        let zip_container = ZipContainerRust::new(path, definition_path);
        Ok(Self { zip_container })
    }

    #[getter]
    fn zip_data(&self) -> PyResult<Option<Vec<u8>>> {
        Ok(self.zip_container.zip_data.clone())
    }

    #[setter]
    fn set_zip_data(&mut self, data: Option<Vec<u8>>) {
        self.zip_container.zip_data = data;
    }

    #[getter]
    fn definition_path(&self) -> PyResult<Option<String>> {
        Ok(self.zip_container.definition_path.clone())
    }

    #[setter]
    fn set_definition_path(&mut self, path: Option<String>) {
        self.zip_container.definition_path = path;
    }

    #[getter]
    fn load_definition_file_str(&self) -> PyResult<String> {
        self.zip_container.load_definition_file_str().map_err(|e| e.into())
    }

    #[setter]
    fn set_files(&mut self, files: Vec<BufFile>) -> PyResult<()> {
        self.zip_container.set_files(files);
        Ok(())
    }

    fn process_files(&mut self) -> PyResult<()> {
        self.zip_container.process_files().map_err(|e| e.into())
    }

    fn get_zip_files(&self) -> PyResult<Vec<BufFile>> {
        self.zip_container.get_zip_files().map_err(|e| e.into())
    }
}

#[pymodule]
fn zip_container(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ZipContainer>()?;
    //m.add_class::<ZipContainer>()?;
    Ok(())
}