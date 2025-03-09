# Plugin Template

## Quick Start

Make sure you have the WebAssembly rust target installed.

```bash
rustup target install wasm32-wasip1
```

Compiling the plugin:

```bash
cargo build --target wasm32-wasip1 --release
```

Finally, zip the `manifest.toml` and `target/wasm32-wasip1/release/<crate_name>.wasm` into an archive.

**NOTE:** They should be placed in the root archive directory.
