# Sxnnyside Project — Command Surface

default:
    @just --list

# Bootstrap all toolchains and pre-requisites
install:
    rustup component add rustfmt clippy
    cargo check --all-targets

# Run the application in local development mode
dev *ARGS:
    cargo run -- {{ ARGS }}

# Produce optimized release build artifacts
build:
    cargo build --release

# Run the complete test suite
test:
    cargo test

# Run correctness/type checking
typecheck:
    cargo check --all-targets

# Run static analysis with strict lints
lint:
    cargo clippy --all-targets -- -D warnings

# Apply deterministic formatting
format:
    cargo fmt

# Verify code style without modifying files
format-check:
    cargo fmt --check

# Audit dependency vulnerabilities and license compliance
deny:
    cargo deny check

# Full quality gate (invoked by CI and pre-commit)
check: format-check lint typecheck test deny
    @echo "✓ All DXQE quality gates passed successfully!"

# Clean build artifacts and caches
clean:
    cargo clean
