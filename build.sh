#!/bin/bash

# Build script for Beam Rust Shaders
# Usage: ./build.sh [contract_name]
#   ./build.sh           - Build all contracts
#   ./build.sh HelloWorld - Build specific contract

set -e

CONTRACT_NAME="$1"

# Function to organize wasm files for a contract
organize_contract() {
    local contract_name="$1"
    local contract_lower=$(echo "$contract_name" | tr '[:upper:]' '[:lower:]')
    local app_wasm="$OUTPUT_DIR/${contract_lower}_app.wasm"
    local contract_wasm="$OUTPUT_DIR/${contract_lower}_contract.wasm"
    local contract_output_dir="$WASM_DIR/$contract_name"
    
    echo "Organizing $contract_name contract..."
    mkdir -p "$contract_output_dir"
    
    if [ -f "$app_wasm" ] && [ -f "$contract_wasm" ]; then
        cp "$app_wasm" "$contract_output_dir/app.wasm"
        cp "$contract_wasm" "$contract_output_dir/contract.wasm"
        echo "  ✓ Copied app.wasm and contract.wasm to $contract_output_dir/"
    else
        echo "  ⚠ Warning: ${contract_lower}_app.wasm or ${contract_lower}_contract.wasm not found in $OUTPUT_DIR"
        return 1
    fi
}

# Function to list available contracts
list_contracts() {
    echo "Available contracts:"
    for contract_dir in Shaders/*/; do
        if [ -d "$contract_dir" ]; then
            echo "  - $(basename "$contract_dir")"
        fi
    done
}

# Create output directory structure
OUTPUT_DIR="target/wasm32-wasi/release"
WASM_DIR="$OUTPUT_DIR/wasm"

# Clean up any existing organized structure
if [ -d "$WASM_DIR" ]; then
    rm -rf "$WASM_DIR"
fi

mkdir -p "$WASM_DIR"

if [ -z "$CONTRACT_NAME" ]; then
    # Build all contracts
    echo "Building all Beam Rust Shaders contracts..."
    echo "Compiling all contracts..."
    cargo build --target wasm32-wasi -r
    
    # Find all contract directories and organize them
    for contract_dir in Shaders/*/; do
        if [ -d "$contract_dir" ]; then
            contract_name=$(basename "$contract_dir")
            organize_contract "$contract_name"
        fi
    done
    
    echo ""
    echo "Build complete! Organized wasm files are in: $WASM_DIR"
    echo "Directory structure:"
    tree "$WASM_DIR" 2>/dev/null || find "$WASM_DIR" -type f
    
else
    # Build specific contract
    CONTRACT_DIR="Shaders/$CONTRACT_NAME"
    
    if [ ! -d "$CONTRACT_DIR" ]; then
        echo "Error: Contract '$CONTRACT_NAME' not found in $CONTRACT_DIR"
        list_contracts
        exit 1
    fi
    
    echo "Building $CONTRACT_NAME contract..."
    echo "Compiling $CONTRACT_NAME contract..."
    contract_lower=$(echo "$CONTRACT_NAME" | tr '[:upper:]' '[:lower:]')
    cargo build --target wasm32-wasi -r -p ${contract_lower}-app -p ${contract_lower}-contract
    
    organize_contract "$CONTRACT_NAME"
    
    echo ""
    echo "Build complete! $CONTRACT_NAME wasm files are in: $WASM_DIR/$CONTRACT_NAME"
fi
