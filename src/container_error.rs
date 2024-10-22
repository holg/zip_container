// container_error.rs
use std::fmt;
use std::string::FromUtf8Error;
use std::error::Error;

/// Enum representing various errors that can occur in the ZipContainer.
#[derive(Debug)]
pub enum ZipContainerError {
    /// Error variant for missing values.
    /// Contains the module path where the error occurred and a message.
    MissingValue {
        module_path: &'static str,
        message: String,
    },
    /// Error variant for I/O errors.
    /// Contains the module path where the error occurred and the source error.
    IOError {
        module_path: &'static str,
        source: std::io::Error,
    },
    /// Error variant for invalid data.
    /// Contains the module path where the error occurred and a message.
    InvalidData {
        module_path: &'static str,
        message: String,
    },
    /// Error variant for Reqwest errors.
    /// Contains the module path where the error occurred and the source error.
    ReqwestError {
        module_path: &'static str,
        source: reqwest::Error,
    },
    UnsupportedOperation {
        module_path: &'static str,
        message: String,
    },
    Utf8Error(FromUtf8Error),
}

impl fmt::Display for ZipContainerError {
    /// Formats the error for display.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZipContainerError::MissingValue { module_path, message } => {
                write!(f, "{}: {}", module_path, message)
            }
            ZipContainerError::IOError { module_path, source } => {
                write!(f, "{}: {}", module_path, source)
            }
            ZipContainerError::InvalidData { module_path, message } => {
                write!(f, "{}: {}", module_path, message)
            }
            ZipContainerError::ReqwestError { module_path, source } => {
                write!(f, "{}: {}", module_path, source)
            }
            ZipContainerError::UnsupportedOperation { module_path, message } => {
                write!(f, "{}: {}", module_path, message)
            }
            ZipContainerError::Utf8Error(e) => {
                write!(f, "UTF-8 error: {}", e)
            }
            // _ => {
            //         write!(f, "Unsupported operation")
            // }
        }
    }
}

impl Error for ZipContainerError {
    /// Returns the source of the error if available.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ZipContainerError::IOError { source, .. } => Some(source),
            ZipContainerError::ReqwestError { source, .. } => Some(source),
            ZipContainerError::Utf8Error(e) => Some(e),
            _ => None,
        }
    }
}

#[macro_export]
macro_rules! function_path {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        name.strip_suffix("::f").unwrap_or(name)
    }};
}


/// Macro to map an expression to an `IOError` variant of `ZipContainerError`.
#[macro_export]
macro_rules! io_err {
    ($expr:expr) => {
        $expr.map_err(|e| $crate::ZipContainerError::IOError {
            module_path: function_path!(),
            source: e.into(),
        })
    };
}

/// Macro to map an expression to a `ReqwestError` variant of `ZipContainerError`.
#[macro_export]
macro_rules! reqwest_err {
    ($expr:expr) => {
        $expr.map_err(|e| $crate::ZipContainerError::ReqwestError {
            module_path: function_path!(),
            source: e,
        })
    };
}

/// Macro to map an expression to an `InvalidData` variant of `ZipContainerError`.
#[macro_export]
macro_rules! invalid_data_err {
    ($expr:expr) => {
        $expr.map_err(|e| $crate::ZipContainerError::InvalidData {
            module_path: function_path!(),
            message: e.to_string(),
        })
    };
}

/// Macro to convert an `Option` to a `Result`, mapping `None` to a `MissingValue` variant of `ZipContainerError`.
#[macro_export]
macro_rules! ok_or_err {
    ($option:expr, $message:expr) => {
        $option.ok_or_else(|| $crate::ZipContainerError::MissingValue {
            module_path: function_path!(),
            message: $message.to_string(),
        })
    };
}
