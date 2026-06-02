"""Demonstrate EvrSortKey usage for database-friendly RPM version ordering."""

from rpm_version import Evr, EvrSortKey

# Create sort keys from individual components
key_a = EvrSortKey.from_values("1", "2.0", "3.fc40")
key_b = EvrSortKey.from_values("1", "3.0", "1.fc40")
assert key_a < key_b
print(f"{key_a!r} < {key_b!r}")

# Or parse from an EVR string
key_c = EvrSortKey.parse("1:2.0-3.fc40")
assert key_a == key_c
print(f"from_values and parse produce identical keys: {key_a == key_c}")

# Sort keys encode RPM version ordering into raw bytes, so standard
# byte comparison produces correct results — useful for database indexes.
versions = ["2.0-1", "1:0.1-1", "1.0~rc1-1", "1.0-1"]
sorted_versions = sorted(versions, key=EvrSortKey.parse)
print(f"\nSorted by sort key: {sorted_versions}")

# The sort key order matches Evr's comparison
sorted_by_evr = sorted(versions, key=Evr.parse)
assert sorted_versions == sorted_by_evr
print(f"Sorted by Evr:      {sorted_by_evr}")

# Evr.sortkey() returns the same bytes as EvrSortKey.from_values()
evr = Evr.parse("1:2.0-3.fc40")
assert evr.sortkey() == EvrSortKey.from_values("1", "2.0", "3.fc40")
