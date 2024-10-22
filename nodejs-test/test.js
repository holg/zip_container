// test.js

// Enable top-level await
(async () => {
    // Import the WASM module
    const wasm = await import('../pkg/zip_container.js');

    // Create an instance of WasmZipContainer
    // Assuming you have a method to create it without any parameters or with defaults
    // Adjust accordingly based on your actual API

    // If you have a method to create from URL
    try {
        const container = await wasm.WasmZipContainer.from_url('https://raw.githubusercontent.com/holg/gldf-rs/refs/heads/master/tests/data/test.gldf', null);

        // Call get_file_names
        const fileNames = await container.get_file_names();
        console.log('File names in the ZIP archive:', fileNames);

        // Read a specific file
        const fileContent = await container.read_file('product.xml');
        console.log('Content of product.xml:', new TextDecoder().decode(fileContent));
    } catch (err) {
        console.error('Error:', err);
    }
})();