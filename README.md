# zip_container

`zip_container` is a versatile Rust library designed to handle ZIP files as containers. It is particularly useful for formats like GLDF or L3D but can be applied to many other formats. The library provides bindings for native Rust applications, Python, and WebAssembly (wasm32), making it a flexible choice for various platforms.

## Features

- **Native Rust Support**: Use \`zip_container\` in your Rust applications to manage ZIP files efficiently.
- **Python Bindings**: Leverage the power of Rust in Python applications using PyO3.
- **WebAssembly (wasm32) Support**: Run \`zip_container\` in the browser or other WebAssembly environments.

## Getting Started

### Native Rust

To use `zip_container` in a Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
zip_container = "0.1.0"
```

### Python

To use `zip_container` in Python, you can install the package from PyPI:
```pip install zip_container```
```bash
### building the python package

```bash
To use `zip_container` in Python, ensure you have `maturin` installed and build the Python package:

1. Install `maturin`:

   ```bash
   pip install maturin
   ```

2. Build the Python package:

   ```bash
   maturin develop
   ```

3. Use the library in your Python code:
    
   ```python
   import zip_container

   # Example usage
   container = zip_container.ZipContainer("path/to/zipfile.zip", None)
   ```

### WebAssembly (wasm32)

To use `zip_container` in a WebAssembly environment, ensure you have the necessary tools installed:

1. Install `wasm-pack`:

   ```bash
   cargo install wasm-pack
   ```

2. Build the WebAssembly package:

   ```bash
   wasm-pack build --target web
   ```

3. Integrate the generated package into your web application.

## Examples

### Rust

```rust
use zip_container::ZipContainer;

fn main() {
let container = ZipContainer::new("path/to/zipfile.zip".to_string(), None);
// Use the container...
}
```

### Python

```python
import zip_container

container = zip_container.ZipContainer("path/to/zipfile.zip", None)
# Use the container...
```

### WebAssembly

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue.

## License

This project is licensed under the MIT License.