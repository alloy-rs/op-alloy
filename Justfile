set positional-arguments
alias t := tests
alias l := lint
alias f := fmtf
alias b := build
alias h := hack
alias c := check
alias e := examples

# default recipe to display help information
default:
  @just --list

# Run all tests
tests: test test-docs

# Test for the native target with all features. By default, excludes online tests.
test:
  cargo nextest run --workspace --all --all-features

# Test the Rust documentation
test-docs:
  cargo test --doc --all

# Lint the workspace for all available targets
lint: lint-native lint-docs

# Lint the workspace
lint-native: fmt-check lint-docs clippy

# Checks the workspace with clippy
clippy:
  cargo +stable clippy --workspace --all-features --all-targets -- -D warnings

# Check the formatting of the workspace
fmt-check:
  cargo +nightly fmt --all -- --check

# Lint the Rust documentation
lint-docs:
  RUSTDOCFLAGS="-D warnings" cargo doc --all --no-deps --document-private-items

# Fixes the formatting of the workspace
fmtf:
  cargo +nightly fmt --all

# Build for the native target
build *args='':
  cargo build --workspace $@

# Checks the workspace with a cfg-check
check:
  cargo check -Zcheck-cfg --workspace

# Runs `cargo hack check` against the workspace
hack:
  cargo hack check --feature-powerset --no-dev-deps --exclude op-alloy --workspace

# Updates the git submodule source
source:
  git submodule update --remote

# Generate file bindings for super-registry
bind:
  @just --justfile ./crates/registry/Justfile bind

# List all available examples and run each one
examples:
  example_list=$(cargo build --example 2>&1); \
  example_list=$(echo "$example_list" | tail -n +3 | sed 's/^[ \t]*//;s/[ \t]*$//'); \
  for example in $example_list; do \
    cargo run --example $example; \
  done
