# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## 0.4.0

- Replace `Nevra::as_normalized_form()` with `Nevra::nevra()`
- Replace `Nevra::nevra()` (the original) with `Nevra::nevra_short()`

## 0.3.0

- Added `set_epoch`, `set_version`, and `set_release` methods to `Evr`
  - Only applies to Rust API
- Accept `None` in place of `epoch`

## 0.2.0

- Added `Hash` implementation for `Evr` and `Nevra` (consistent with `PartialEq` epoch normalization)
- Added optional `serde` feature for `Serialize`/`Deserialize` on `Evr` and `Nevra`
- Added `Requirement` type for version requirement matching (e.g. `foo >= 1:2.0-1`)
- Added `evr_sort` and `nevra_sort` Python functions for bulk sorting entirely in Rust with FFI overhead and allocations

## 0.1.0

- Initial Release, split from https://github.com/rpm-rs/rpm-rs
- Identical to the previous functionality
