# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

- `Evr` equality and hashing now agree with its ordering: EVRs that compare as equal, such as `1.01-1` and `1.1-1`, are `==` and hash the same. Previously `==` compared the strings, so `BTreeSet` and `HashSet` disagreed about duplicates
  - Also applies to `Nevra`, `Requirement` and the Python API
- `Nevra` no longer compares as equal when the names or arches differ only in separators, such as `foo-bar` and `foo_bar`: these are now ordered by the strings themselves. Previously they compared as equal but were not `==`
- Sortkeys of numeric segments longer than 255 digits now sort by their value; previously their length wrapped around in the length byte. Keys of shorter segments are unchanged

## 0.5.1

- Bump PyO3 dependency version

## 0.5.0

- Added `EvrSortKey` newtype wrapping `Vec<u8>` with `from_values()`, `parse()`, and `as_bytes()` methods
- Added `Evr::sortkey()` returning `EvrSortKey`
- Added `version_sortkey()` for encoding a single version/release component
- Python: Added `EvrSortKey` class with `from_values()` and `parse()` classmethods returning `bytes`
- Python: Added `Evr.sortkey()` method
- Python: Added `version_sortkey()` function

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
