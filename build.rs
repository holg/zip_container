#[cfg(feature = "python")]
fn main() {
    // Add the extension module link arguments only when the 'python' feature is enabled.
    use pyo3_build_config;
    pyo3_build_config::add_extension_module_link_args();
}

#[cfg(not(feature = "python"))]
fn main() {
    // If the 'python' feature is not enabled, we do nothing in the build script.
}
