.PHONY: help preflight publish test check build clean doc release-check release release-patch release-minor
.PHONY: fmt clippy example

# Define examples
EXAMPLES := examples/basic

help:
	@echo "Available targets:"
	@echo "  make check          - Run cargo check on all crates"
	@echo "  make test           - Run all tests"
	@echo "  make build          - Build all crates"
	@echo "  make doc            - Generate documentation"
	@echo "  make clean          - Clean build artifacts"
	@echo "  make fmt            - Format all code"
	@echo "  make clippy         - Run clippy with auto-fix"
	@echo "  make preflight      - Run all checks before committing/publishing"
	@echo ""
	@echo "Example targets:"
	@echo "  make example        - Run the basic example"
	@echo ""
	@echo "Release targets:"
	@echo "  make release-check  - Dry-run release with cargo-release"
	@echo "  make release        - Release patch version (0.x.y -> 0.x.y+1)"
	@echo "  make release-patch  - Release patch version (same as release)"
	@echo "  make release-minor  - Release minor version (0.x.y -> 0.x+1.0)"
	@echo "  make publish        - Publish to crates.io"

check:
	@echo "🔍 Checking all crates..."
	cargo check --workspace --all-targets --all-features

test:
	@echo "🧪 Running tests..."
	cargo test --workspace --all-targets --all-features
	cargo test --workspace --doc --all-features

build:
	@echo "🔨 Building all crates..."
	cargo build --workspace --all-features

doc:
	@echo "📚 Generating documentation..."
	cargo doc --workspace --all-features --no-deps --open

clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean

fmt:
	@echo "🎨 Formatting code..."
	cargo fmt --all

clippy:
	@echo "� Running clippy (auto-fix)..."
	cargo clippy --workspace --all-targets --all-features --fix --allow-dirty --allow-staged -- -D warnings

example:
	@echo "� Running basic example..."
	cargo run -p example-basic

preflight:
	@echo "🚦 Running preflight checks for the entire workspace..."
	@echo ""
	@echo "1️⃣  Formatting code..."
	cargo fmt --all
	@echo ""
	@echo "2️⃣  Running clippy (auto-fix)..."
	cargo clippy --workspace --all-targets --all-features --fix --allow-dirty --allow-staged -- -D warnings
	@echo ""
	@echo "3️⃣  Running tests..."
	cargo test --workspace --all-targets --all-features
	cargo test --workspace --doc --all-features
	@echo ""
	@echo "✅ All preflight checks passed!"

release-check:
	@echo "🔍 Dry-run release with cargo-release..."
	@echo ""
	@echo "Note: Install cargo-release if not already installed:"
	@echo "  cargo install cargo-release"
	@echo ""
	@echo "Checking patch release (0.x.y -> 0.x.y+1)..."
	cargo release patch

release-patch: preflight
	@echo "🚀 Releasing PATCH version with cargo-release..."
	@echo ""
	@echo "This will:"
	@echo "  - Update version numbers (0.x.y -> 0.x.y+1)"
	@echo "  - Create git commit and tag"
	@echo "  - (Publish step is manual, see make publish)"
	@echo ""
	@read -p "Continue? [y/N] " confirm && [ "$$confirm" = "y" ] || exit 1
	cargo release patch --execute --no-confirm

release-minor: preflight
	@echo "🚀 Releasing MINOR version with cargo-release..."
	@echo ""
	@echo "This will:"
	@echo "  - Update version numbers (0.x.y -> 0.x+1.0)"
	@echo "  - Create git commit and tag"
	@echo "  - (Publish step is manual, see make publish)"
	@echo ""
	@read -p "Continue? [y/N] " confirm && [ "$$confirm" = "y" ] || exit 1
	cargo release minor --execute --no-confirm

release: release-patch

publish: preflight
	@echo ""
	@echo "🚀 Starting sequential publish process..."
	@echo ""

	@echo "--- Step 1: Publishing schema-bridge-macro ---"
	@echo "  Running dry-run for schema-bridge-macro..."
	cargo publish -p schema-bridge-macro --dry-run --allow-dirty

	@echo "  ✓ Dry-run successful for schema-bridge-macro"
	@echo "  Publishing schema-bridge-macro to crates.io..."
	cargo publish -p schema-bridge-macro --allow-dirty

	@echo ""
	@echo "✅ schema-bridge-macro published successfully!"
	@echo ""
	@echo "⏳ Waiting 30 seconds for crates.io index to update..."
	sleep 30

	@echo ""
	@echo "--- Step 2: Publishing schema-bridge-core ---"
	@echo "  Running dry-run for schema-bridge-core..."
	cargo publish -p schema-bridge-core --dry-run --allow-dirty

	@echo "  ✓ Dry-run successful for schema-bridge-core"
	@echo "  Publishing schema-bridge-core to crates.io..."
	cargo publish -p schema-bridge-core --allow-dirty

	@echo ""
	@echo "✅ schema-bridge-core published successfully!"
	@echo ""
	@echo "⏳ Waiting 30 seconds for crates.io index to update..."
	sleep 30

	@echo ""
	@echo "--- Step 3: Publishing schema-bridge (main) ---"
	@echo "  Running dry-run for schema-bridge..."
	cargo publish -p schema-bridge --dry-run --allow-dirty

	@echo "  ✓ Dry-run successful for schema-bridge"
	@echo "  Publishing schema-bridge to crates.io..."
	cargo publish -p schema-bridge --allow-dirty

	@echo ""
	@echo "✅ schema-bridge published successfully!"
	@echo ""
	@echo "🎉 All crates have been successfully published to crates.io!"
