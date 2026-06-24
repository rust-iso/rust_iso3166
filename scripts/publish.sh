#!/usr/bin/env bash
#
# Release rust_iso3166 to both registries:
#   - crates.io  (the Rust crate, via `cargo publish`)
#   - npm        (the wasm package in ./pkg, via `wasm-pack` + `npm publish`)
#
# Steps:
#   1. sanity checks (required tools, clean working tree)
#   2. run the full test suite (`cargo test --all-features`)
#   3. publish the crate to crates.io
#   4. build the wasm/npm package (scripts/build-wasm.sh)
#   5. publish the npm package from ./pkg
#
# Pass --dry-run to exercise cargo/npm in dry-run mode without publishing.
#
# Usage:
#   scripts/publish.sh [--dry-run]
#
set -euo pipefail

cd "$(dirname "$0")/.."

DRY_RUN=0
if [ "${1:-}" = "--dry-run" ]; then
    DRY_RUN=1
fi

VERSION=$(grep -m1 '^version' Cargo.toml | sed -E 's/.*"([^"]+)".*/\1/')
echo ">> Releasing rust_iso3166 v${VERSION} (dry-run=${DRY_RUN})"

# 1. sanity checks
for tool in cargo wasm-pack npm git; do
    command -v "$tool" >/dev/null 2>&1 || { echo "error: '$tool' not found in PATH" >&2; exit 1; }
done

if [ -n "$(git status --porcelain)" ]; then
    echo "error: working tree is dirty; commit or stash changes before publishing." >&2
    exit 1
fi

# 2. tests (exercise every feature: cli + serde)
cargo test --all-features

# 3. crates.io
if [ "$DRY_RUN" = 1 ]; then
    cargo publish --dry-run
else
    cargo publish
fi

# 4. build the wasm / npm package (wasm-pack syncs pkg/package.json from Cargo.toml)
scripts/build-wasm.sh

# 5. npm
if [ "$DRY_RUN" = 1 ]; then
    (cd pkg && npm publish --dry-run --access public)
else
    (cd pkg && npm publish --access public)
fi

echo ">> Done: rust_iso3166 v${VERSION}"
