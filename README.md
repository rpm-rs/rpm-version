## Description

A library for dealing with RPM versions (NEVRA, EVR) correctly. Sort algorithm is identical to RPM.

## Usage

### In Rust

```rust
use std::cmp::Ordering;
use rpm_version::{Evr, Nevra, rpm_evr_compare};

// Compare EVR strings directly
assert_eq!(rpm_evr_compare("1.2.3-4", "1.2.3-5"), Ordering::Less);
assert_eq!(rpm_evr_compare("2:1.0-1", "1:9.9-1"), Ordering::Greater);

// Or use the Evr struct for structured comparisons
let v1 = Evr::parse("1:2.3.4-5");
let v2 = Evr::parse("1:2.3.4-6");
assert!(v1 < v2);

// Tilde (~) denotes a pre-release: sorts before the version without it
assert!(Evr::parse("1.0~rc1") < Evr::parse("1.0"));
// Caret (^) denotes a post-release snapshot: sorts after the base version
assert!(Evr::parse("1.0") < Evr::parse("1.0^git1"));
assert!(Evr::parse("1.0^git1") < Evr::parse("1.1"));

// Full NEVRA (Name-Epoch-Version-Release-Architecture) parsing
let nevra = Nevra::parse("foo-1:2.3.4-5.x86_64");
println!("{} {} {}", nevra.name(), nevra.version(), nevra.arch()); // foo 2.3.4 x86_64

// Sorting NEVRAs
let mut packages = vec![
    Nevra::parse("foo-2.0-1.x86_64"),
    Nevra::parse("bar-1.0-1.x86_64"),
    Nevra::parse("foo-1.0-1.x86_64"),
];
packages.sort();
assert_eq!(packages[0].name(), "bar");
assert_eq!(packages[2].to_string(), "foo-2.0-1.x86_64");

// Evr and Nevra are hashable
use std::collections::HashSet;
let mut seen = HashSet::new();
seen.insert(Nevra::parse("foo-1.0-1.x86_64"));

// Version requirement matching
use rpm_version::{Requirement, ReqOperator};
let req = Requirement::with_constraint("foo", ReqOperator::Ge, Evr::parse("2.0-1"));
assert!(req.satisfies("foo", &Evr::parse("2.0-1")));
assert!(req.satisfies("foo", &Evr::parse("3.0-1")));
assert!(!req.satisfies("foo", &Evr::parse("1.0-1")));
```

### In Python

```python
from rpm_version import Evr, Nevra, evr_compare, evr_sort, nevra_sort

# Compare EVR strings directly
assert evr_compare("1.2.3-4", "1.2.3-5") == -1
assert evr_compare("2:1.0-1", "1:9.9-1") == 1

# Or use the Evr object for structured comparisons
v1 = Evr.parse("1:2.3.4-5")
v2 = Evr.parse("1:2.3.4-6")
assert v1 < v2

# Tilde (~) denotes a pre-release: sorts before the version without it
assert Evr.parse("1.0~rc1-1") < Evr.parse("1.0-1")
# Caret (^) denotes a post-release snapshot: sorts after the base version
assert Evr.parse("1.0-1") < Evr.parse("1.0^git1-1")
assert Evr.parse("1.0^git1-1") < Evr.parse("1.1-1")

# Full NEVRA (Name-Epoch-Version-Release-Architecture) parsing
nevra = Nevra.parse("foo-1:2.3.4-5.x86_64")
print(f"{nevra.name} {nevra.version} {nevra.arch}")  # foo 2.3.4 x86_64

# Bulk sorting entirely in Rust (avoids per-comparison FFI overhead)
sorted_versions = evr_sort(["2.0-1", "1.0-1", "1:0.5-1", "3.0-1"])
assert sorted_versions == ["1.0-1", "2.0-1", "3.0-1", "1:0.5-1"]

sorted_packages = nevra_sort(["foo-2.0-1.x86_64", "bar-1.0-1.x86_64", "foo-1.0-1.x86_64"])
assert sorted_packages == ["bar-1.0-1.x86_64", "foo-1.0-1.x86_64", "foo-2.0-1.x86_64"]

# Evr and Nevra are hashable
seen = {nevra}

# Version requirement matching
from rpm_version import Requirement, ReqOperator
req = Requirement("foo", ReqOperator.GE, Evr.parse("2.0-1"))
assert req.satisfies("foo", Evr.parse("2.0-1"))
assert req.satisfies("foo", Evr.parse("3.0-1"))
assert not req.satisfies("foo", Evr.parse("1.0-1"))

# String operators also work
req = Requirement("foo", ">=", Evr.parse("2.0-1"))
```
