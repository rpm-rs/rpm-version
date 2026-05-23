## Description

A library for dealing with RPM versions (NEVRA, EVR) correctly.

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

// Full NEVRA (Name-Epoch-Version-Release-Architecture) parsing
let nevra = Nevra::parse("foo-1:2.3.4-5.x86_64");
println!("{} {} {}", nevra.name(), nevra.version(), nevra.arch()); // foo 2.3.4 x86_64
```

### In Python

```python
from rpm_version import Evr, Nevra, evr_compare

# Compare EVR strings directly
assert evr_compare("1.2.3-4", "1.2.3-5") == -1
assert evr_compare("2:1.0-1", "1:9.9-1") == 1

# Or use the Evr object for structured comparisons
v1 = Evr.parse("1:2.3.4-5")
v2 = Evr.parse("1:2.3.4-6")
assert v1 < v2

# Full NEVRA (Name-Epoch-Version-Release-Architecture) parsing
nevra = Nevra.parse("foo-1:2.3.4-5.x86_64")
print(f"{nevra.name} {nevra.version} {nevra.arch}")  # foo 2.3.4 x86_64
```
