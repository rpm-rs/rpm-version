
## Description

A library for dealing with RPM versions (NEVRA, EVR) correctly. Sort algorithm is identical to RPM.

## Usage

```python
from rpm_version import Evr, Nevra, evr_compare, evr_sort, nevra_sort

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

# Tilde (~) denotes a pre-release: sorts before the version without it
assert Evr.parse("1.0~rc1-1") < Evr.parse("1.0-1")
# Caret (^) denotes a post-release snapshot: sorts after the base version
assert Evr.parse("1.0-1") < Evr.parse("1.0^git1-1")
assert Evr.parse("1.0^git1-1") < Evr.parse("1.1-1")

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

# Sort keys: encode RPM version ordering into raw bytes.
# Useful in databases (in-database RPM version ordering)
from rpm_version import EvrSortKey
key_a = EvrSortKey.from_values("1", "2.0", "3.fc40")
key_b = EvrSortKey.parse("1:3.0-1.fc40")
assert key_a < key_b
# Evr.sortkey() returns the same thing
assert Evr.parse("1:2.0-3.fc40").sortkey() == key_a
# Use as a sort key for sorted()
sorted(["2.0-1", "1:0.1-1", "1.0-1"], key=EvrSortKey.parse)
```
