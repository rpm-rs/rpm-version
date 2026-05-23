# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

- Added `Hash` implementation for `Evr` and `Nevra` (consistent with `PartialEq` epoch normalization)
- Added optional `serde` feature for `Serialize`/`Deserialize` on `Evr` and `Nevra`
- Added `Requirement` type for version requirement matching (e.g. `foo >= 1:2.0-1`)
- Added `evr_sort` and `nevra_sort` Python functions for bulk sorting entirely in Rust with FFI overhead and allocations

## 0.1.0

- Initial Release, split from https://github.com/rpm-rs/rpm-rs
- Identical to the previous functionality
