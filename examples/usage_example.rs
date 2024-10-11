// examples/usage_example.rs
extern crate zip_container;
use zip_container::{ZipContainer, ZipContainerResult, ZipContainerTrait};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main() {
    wasm_bindgen_futures::spawn_local(async {
        if let Err(e) = run().await {
            // Handle error
            web_sys::console::error_1(&format!("Error: {:?}", e).into());
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> ZipContainerResult<()> {
    run().await
}

async fn run() -> ZipContainerResult<()> {
    // Create a new ZipContainer instance
    let mut zip_container = ZipContainer::new("https://raw.githubusercontent.com/holg/gldf-rs/refs/heads/master/tests/data/test.gldf".to_string(), Some("product.xml".to_string()));
    let _ = &zip_container.set_files(zip_container.get_zip_files()?);
    let _ = &zip_container.process_files().await?;
    // Write zip_data to a local file
    // if let Some(ref zip_data) = zip_container.zip_data {
    //     let mut file = File::create("output.zip").expect("Failed to create output.zip");
    //     file.write_all(zip_data).expect("Failed to write zip_data to output.zip");
    //     println!("zip_data has been written to output.zip");
    // }
    // Access the loaded files
    if let Some(ref files) = zip_container.files {
        for file in files {
            println!("Loaded file: {:?}", file.name);
        }
    }
    let product_xml_string = zip_container.load_definition_file_str().await?;
    let product_xml = product_xml_string.as_str();
    println!("product.xml: {}", product_xml);
    Ok(())
}
