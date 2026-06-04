# ─── just <recipe> ────────────────────────────────────────────────────────────
# Install just: cargo install just
# Run all checks: just ci

set shell := ["bash", "-cu"]

# Default: show available recipes
default:
    @just --list

# ── Development ───────────────────────────────────────────────────────────────

# Build all crates in debug mode
build:
    cargo build --workspace

# Build in release mode
build-release:
    cargo build --workspace --release

# Run the HTTP server (dev)
run:
    RUST_LOG=debug cargo run -p server

# Run all tests across the workspace
test:
    cargo test --workspace

# Run tests with output visible
test-verbose:
    cargo test --workspace -- --nocapture

# ── Code quality ──────────────────────────────────────────────────────────────

# Format all code (stable)
fmt:
    cargo fmt --all

# Format all code (nightly – enables extra options in rustfmt.toml)
fmt-nightly:
    cargo +nightly fmt --all

# Check formatting without modifying files (stable)
fmt-check:
    cargo fmt --all -- --check

# Check formatting without modifying files (nightly)
fmt-check-nightly:
    cargo +nightly fmt --all -- --check

# Type-check without codegen (fast)
check:
    cargo check --workspace

# Run Clippy (all features, deny warnings in CI)
clippy:
    cargo clippy --workspace --all-features -- -D warnings

# Spell-check the codebase
typos:
    typos

# ── Security ──────────────────────────────────────────────────────────────────

# Audit dependencies for known vulnerabilities
audit:
    cargo audit

# Run cargo-deny checks (licenses + advisories + bans)
deny:
    cargo deny check

# Run cargo-vet supply-chain verification
vet:
    cargo vet

# ── CI gate (run all checks locally before pushing) ──────────────────────────
ci: fmt-check check clippy test typos audit deny
    @echo "✓ all checks passed"
