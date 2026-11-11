# Ruby bindings

## Description

The `rpm-version` Ruby bindings provide RPM compatible parsing, comparison,
and sorting for EVR (Epoch-Version-Release) and NEVRA
(Name-Epoch-Version-Release-Architecture) values. The native extension uses
the same Rust implementation as the `rpm-version` crate.

## Usage

```ruby
require "rpm_version"

# Compare EVR strings directly
raise unless RPMVersion.evr_compare("1.2.3-4", "1.2.3-5") == -1
raise unless RPMVersion.evr_compare("2:1.0-1", "1:9.9-1") == 1

# Or use the Evr object for structured comparisons
v1 = RPMVersion::Evr.parse("1:2.3.4-5")
v2 = RPMVersion::Evr.parse("1:2.3.4-6")
raise unless v1 < v2

# Full NEVRA (Name-Epoch-Version-Release-Architecture) parsing
nevra = RPMVersion::Nevra.parse("foo-1:2.3.4-5.x86_64")
puts "#{nevra.name} #{nevra.version} #{nevra.arch}" # foo 2.3.4 x86_64

# Tilde (~) denotes a pre-release: sorts before the version without it
raise unless RPMVersion::Evr.parse("1.0~rc1-1") < RPMVersion::Evr.parse("1.0-1")
# Caret (^) denotes a post-release snapshot: sorts after the base version
raise unless RPMVersion::Evr.parse("1.0-1") < RPMVersion::Evr.parse("1.0^git1-1")
raise unless RPMVersion::Evr.parse("1.0^git1-1") < RPMVersion::Evr.parse("1.1-1")

# Bulk sorting entirely in Rust (avoids per-comparison FFI overhead)
sorted_versions = RPMVersion.evr_sort(["2.0-1", "1.0-1", "1:0.5-1", "3.0-1"])
raise unless sorted_versions == ["1.0-1", "2.0-1", "3.0-1", "1:0.5-1"]

sorted_packages = RPMVersion.nevra_sort([
  "foo-2.0-1.x86_64",
  "bar-1.0-1.x86_64",
  "foo-1.0-1.x86_64",
])
raise unless sorted_packages == [
  "bar-1.0-1.x86_64",
  "foo-1.0-1.x86_64",
  "foo-2.0-1.x86_64",
]

# Evr and Nevra are hashable
seen = { nevra => true }

# Version requirement matching
requirement = RPMVersion::Requirement.new(
  "foo",
  RPMVersion::ReqOperator::GE,
  RPMVersion::Evr.parse("2.0-1"),
)
raise unless requirement.satisfies?("foo", RPMVersion::Evr.parse("2.0-1"))
raise unless requirement.satisfies?("foo", RPMVersion::Evr.parse("3.0-1"))
raise if requirement.satisfies?("foo", RPMVersion::Evr.parse("1.0-1"))

# String operators also work
requirement = RPMVersion::Requirement.new(
  "foo",
  ">=",
  RPMVersion::Evr.parse("2.0-1"),
)

# Sort keys: encode RPM version ordering into raw bytes.
# Useful in databases (in-database RPM version ordering)
key_a = RPMVersion::EvrSortKey.from_values("1", "2.0", "3.fc40")
key_b = RPMVersion::EvrSortKey.parse("1:3.0-1.fc40")
# Evr.sortkey() returns the same thing
raise unless RPMVersion::Evr.parse("1:2.0-3.fc40").sortkey == key_a
raise unless key_a < key_b
# Use as a sort key for sorted()
sorted = ["2.0-1", "1:0.1-1", "1.0-1"].sort_by do |evr|
  RPMVersion::EvrSortKey.parse(evr)
end
```
