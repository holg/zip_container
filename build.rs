use std::env;
// use std::process::Command;
#[cfg(feature = "python")]
fn main() {
    // Add the extension module link arguments only when the 'python' feature is enabled.
    use pyo3_build_config;
    pyo3_build_config::add_extension_module_link_args();
    if env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "arm" {
        println!("cargo:rustc-cfg=__ARM_ARCH");
    }
}

#[cfg(not(feature = "python"))]
fn main() {
    if env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "arm" {
        println!("cargo:rustc-cfg=__ARM_ARCH");
    }
}
