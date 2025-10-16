# Beam Shader in Rust language (in development)
Beam shader with C++ bvm bindings for Rust

## Project Structure
```
root/
├── common/           # Common files that contracts need
├── Shaders/          # Individual contracts
│   └── HelloWorld/   # Example HelloWorld contract
│       ├── app/      # Manager/interface part
│       └── contract/ # Contract implementation
└── Cargo.toml        # Workspace configuration
```

## Usage
1. Install `rustup` on your system. See rust installation instructions [here](https://forge.rust-lang.org/infra/other-installation-methods.html#other-ways-to-install-rustup).
2. Install rust toolchain:
  `$ rustup toolchain install stable`
3. Add wasm32-wasi target
  `$ rustup target add wasm32-wasi`

### Building Contracts

#### Option 1: Using Make (Recommended)
```bash
# Build all contracts
$ make

# Build a specific contract
$ make build-contract CONTRACT=HelloWorld

# List available contracts
$ make list-contracts

# Clean build artifacts
$ make clean
```

#### Option 2: Using Build Script
```bash
# Build all contracts
$ ./build.sh

# Build a specific contract
$ ./build.sh HelloWorld
```

#### Option 3: Using Cargo Directly
```bash
# Build all contracts (files will be mixed in target/wasm32-wasi/release/)
$ cargo build --target wasm32-wasi -r

# Build a specific contract
$ cargo build --target wasm32-wasi -r -p app -p contract
```

### Output Structure
When using the build scripts or Make, compiled wasm files are organized by contract:
```
target/wasm32-wasi/release/wasm/
├── HelloWorld/
│   ├── app.wasm
│   └── contract.wasm
└── [OtherContract]/
    ├── app.wasm
    └── contract.wasm
```

After that you can use `app.wasm` and `contract.wasm` files in the same way you use it in Beam's contracts (see https://github.com/BeamMW/shader-sdk/wiki/Running-Beam-Shaders-using-CLI-Wallet).

## Adding New Contracts
To add a new contract:
1. Create a new directory under `Shaders/` (e.g., `Shaders/MyContract/`)
2. Add `app/` and `contract/` subdirectories with their respective `Cargo.toml` and `src/lib.rs` files
3. Update the root `Cargo.toml` workspace members to include your new contract paths
4. Ensure both `app` and `contract` depend on the `common` crate