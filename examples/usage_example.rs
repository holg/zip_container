// examples/usage_example.rs

extern crate zip_container;
#[cfg(not(target_arch = "wasm32"))]
use zip_container::{ZipContainer, ZipContainerResult, ZipContainerTrait};

#[cfg(not(target_arch = "wasm32"))]
fn main() -> ZipContainerResult<()> {
    // Create a new ZipContainer instance
    let zip_container = ZipContainer::new("https://raw.githubusercontent.com/holg/gldf-rs/refs/heads/master/tests/data/test.gldf".to_string(), Some("product.xml".to_string()));
    for file in zip_container.get_zip_files()?.iter(){
            println!("Loaded file name: {}, size: {}, path: {}, file_id: {}",
                file.name.clone().expect("Failed to get file name"),
                file.size.clone().expect("Failed to get file size"),
                file.path.clone().expect("Failed to get file path"),
                file.file_id.clone().unwrap_or("Failed to get file id".to_string())
            );
    }
    let product_xml_string = zip_container.load_definition_file_str()?;
    let product_xml = product_xml_string.as_str();
    println!("product.xml: {}", product_xml);
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub fn main() {
    use web_sys::{console};
    console::log_1(&"This example is not supported in WASM.".into());
}

