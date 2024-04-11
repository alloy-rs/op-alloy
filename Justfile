set positional-arguments

# default recipe to display help information
default:
  @just --list

# Run all ci checks for the native target
ci: lint-native lint-docs test

# Test for the native target
test *args='':
  cargo nextest run --workspace --all $@

# Lint the workspace for all available targets
lint: lint-native lint-docs

# Fixes the formatting of the workspace
fmt-native-fix:
  cargo +nightly fmt --all

# Check the formatting of the workspace
fmt-native-check:
  cargo +nightly fmt --all -- --check

# Lint the workspace
lint-native: fmt-native-check
  cargo +nightly clippy --workspace --all --all-features --all-targets -- -D warnings

# Lint the Rust documentation
lint-docs:
  RUSTDOCFLAGS="-D warnings" cargo doc --all --no-deps --document-private-items 

# Test the Rust documentation
test-docs:
  cargo test --doc --all --locked

# Build the workspace for all available targets
build: build-native

# Build for the native target
build-native *args='':
  cargo build --workspace --all $@
