# Makefile for Beam Rust Shaders

.PHONY: all clean build help list-contracts

# Default target
all: build

# Build all contracts and organize output
build:
	@echo "Building all contracts..."
	@./build.sh

# Build a specific contract
# Usage: make build-contract CONTRACT=HelloWorld
build-contract:
	@if [ -z "$(CONTRACT)" ]; then \
		echo "Usage: make build-contract CONTRACT=<contract_name>"; \
		echo "Available contracts:"; \
		$(MAKE) list-contracts; \
		exit 1; \
	fi
	@./build.sh $(CONTRACT)

# List all available contracts
list-contracts:
	@echo "Available contracts:"
	@for contract_dir in Shaders/*/; do \
		if [ -d "$$contract_dir" ]; then \
			echo "  - $$(basename "$$contract_dir")"; \
		fi; \
	done

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@cargo clean
	@if [ -d "target/wasm32-wasi/release/wasm" ]; then \
		rm -rf target/wasm32-wasi/release/wasm; \
	fi

# Show help
help:
	@echo "Beam Rust Shaders Build System"
	@echo ""
	@echo "Available targets:"
	@echo "  all              - Build all contracts (default)"
	@echo "  build            - Build all contracts and organize output"
	@echo "  build-contract   - Build a specific contract (use CONTRACT=<name>)"
	@echo "  list-contracts   - List all available contracts"
	@echo "  clean            - Clean all build artifacts"
	@echo "  help             - Show this help message"
	@echo ""
	@echo "Examples:"
	@echo "  make                    # Build all contracts"
	@echo "  make build-contract CONTRACT=HelloWorld  # Build HelloWorld contract"
	@echo "  make list-contracts     # List available contracts"
	@echo "  make clean              # Clean build artifacts"
