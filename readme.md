# Rust-NodeJS Integration Example

This repository demonstrates how to integrate Rust and Node.js using WebAssembly.

## Structure

- **rust/**: Contains the Rust code.
- **node/**: Contains the Node.js code that uses the WebAssembly module generated from the Rust code.

## Setup

### Rust

1. Navigate to the `rust` directory:

   ```bash
   cd rust 
   ```

2. Build the Rust project using wasm-pack:

    ```bash
   wasm-pack build --target nodejs 
   ```

3. Copy the pkg directory to the nodejs directory.


### Node

1. Navigate to the `node` directory:

   ```bash
   cd ../node 
   ```

2. Build the Rust project using wasm-pack:

    ```bash
   npm install 
   ```

2. Run the Node.js script:

    ```bash
   ts-node ./index.ts
   ```
