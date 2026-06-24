# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-06-24

### Added
- Optional `serde` support: `Serialize`/`Deserialize` for `CountryCode`,
  `iso3166_2::Subdivision` and `iso3166_3::CountryCode3`, behind the new
  `serde` feature. Values serialize to their canonical code string
  (alpha-2 / subdivision code / alpha-4) and deserialize case-insensitively
  via the existing `from_*` lookups. Enable with
  `cargo add rust_iso3166 -F serde`. Numeric-code (de)serialization is not
  supported. (#13)
- `cli` feature gating the command-line tool; the binary is now named
  `iso3166`. (#14)
- `scripts/build-wasm.sh` and `scripts/publish.sh` for building the
  WebAssembly/npm package and releasing to crates.io + npm.
- `LICENSE` file (Apache-2.0).

### Changed
- **Breaking:** `CountryCode.numeric` is now `u16` instead of `i32` (also the
  `numeric()` getter, the `from_numeric` parameter, and the `ALL_NUMERIC`
  slice). ISO 3166-1 numeric codes are non-negative and at most `999`, so `u16`
  models the domain precisely. (#9)
- Refreshed country short names to the current ISO 3166-1 values
  (`Turkey` → `Türkiye`, `Netherlands` → `Netherlands, Kingdom of the`) and
  removed stray Wikipedia footnote markers from several names (Bonaire…,
  Saint Helena…, Svalbard and Jan Mayen, United States Minor Outlying Islands).
- **Breaking:** upgraded `phf` from 0.11 to 0.14. As `phf::Map` appears in the
  public API (the `*_MAP` constants), this is a public-dependency major bump.
- **Breaking:** the CLI is no longer built by default — build/install it with
  `--features cli`. `prettytable-rs` is now an optional dependency pulled in
  only by that feature, so default library builds no longer depend on it. (#14)
- **Breaking:** minimum supported Rust version is now **1.85** (required by
  `phf` 0.14 / edition 2024 in the dependency tree).
- Refreshed dependency floors: `serde_json` 1.0.150 (dev), `wasm-bindgen`
  0.2.100, `js-sys` 0.3.77, `wasm-bindgen-test` 0.3.50 (dev).
- The `serde` impls live in the code generator templates (`scripts/*.py`) and
  the `src/*.rs` files are regenerated from them, so the feature survives future
  regeneration.

### Fixed
- Silenced `unexpected_cfgs` warnings for the custom `direct_wasm` cfg via a
  `[lints.rust]` `check-cfg` entry.

[0.2.0]: https://github.com/rust-iso/rust_iso3166/releases/tag/0.2.0
