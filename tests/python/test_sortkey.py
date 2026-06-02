"""Tests for EvrSortKey, version_sortkey, and Evr.sortkey."""

from rpm_version import Evr, EvrSortKey, version_sortkey


class TestEvrSortkey:
    def test_epoch_none_equals_zero(self):
        assert EvrSortKey.from_values(None, "1.2.3", "45") == EvrSortKey.from_values(
            "0", "1.2.3", "45"
        )

    def test_epoch_empty_equals_zero(self):
        assert EvrSortKey.from_values("", "1.2.3", "45") == EvrSortKey.from_values(
            "0", "1.2.3", "45"
        )

    def test_higher_epoch_wins(self):
        assert EvrSortKey.from_values("", "1.2.3", "45") < EvrSortKey.from_values(
            "1", "1.2.3", "45"
        )

    def test_epoch_dominates_version(self):
        assert EvrSortKey.from_values("", "4.2.3", "45") < EvrSortKey.from_values(
            "1", "1.2.3", "45"
        )

    def test_version_ordering(self):
        assert EvrSortKey.from_values("", "1.2.3", "45") < EvrSortKey.from_values(
            "", "1.2.4", "45"
        )

    def test_release_ordering(self):
        assert EvrSortKey.from_values("", "1.2.3", "3") < EvrSortKey.from_values(
            "", "1.2.3", "10"
        )

    def test_tilde_version(self):
        assert EvrSortKey.from_values("", "1.0~rc1", "1") < EvrSortKey.from_values(
            "", "1.0", "1"
        )

    def test_caret_version(self):
        assert EvrSortKey.from_values("", "1.0", "1") < EvrSortKey.from_values(
            "", "1.0^git1", "1"
        )
        assert EvrSortKey.from_values("", "1.0^git1", "1") < EvrSortKey.from_values(
            "", "1.0.1", "1"
        )


class TestEvrSortKeyParse:
    def test_matches_from_values(self):
        assert EvrSortKey.parse("2:1.0~rc1-3.fc40") == EvrSortKey.from_values(
            "2", "1.0~rc1", "3.fc40"
        )

    def test_no_epoch(self):
        assert EvrSortKey.parse("1.0-1") == EvrSortKey.from_values("", "1.0", "1")

    def test_ordering(self):
        assert EvrSortKey.parse("1.0-1") < EvrSortKey.parse("2.0-1")

    def test_epoch_dominates(self):
        assert EvrSortKey.parse("1:0.1-1") > EvrSortKey.parse("9.9-1")


class TestEvrSortkeyMethod:
    def test_matches_function(self):
        e = Evr.parse("2:1.0~rc1-3.fc40")
        assert e.sortkey() == EvrSortKey.from_values("2", "1.0~rc1", "3.fc40")

    def test_sortkey_ordering_matches_evr_ordering(self):
        cases = [
            ("1.0-1", "2.0-1"),
            ("1.0~rc1-1", "1.0-1"),
            ("1.0-1", "1.0^git1-1"),
            ("1.2.3-45", "1:1.2.3-45"),
        ]
        for a, b in cases:
            ea = Evr.parse(a)
            eb = Evr.parse(b)
            assert (ea.sortkey() < eb.sortkey()) == (ea < eb), f"{a} vs {b}"

    def test_sortkey_can_sort_list(self):
        versions = ["1.0^git1-1", "1.0-1", "1.0~rc1-1", "1.1-1"]
        evrs = [Evr.parse(v) for v in versions]
        sorted_by_key = sorted(evrs, key=lambda e: e.sortkey())
        sorted_by_cmp = sorted(evrs)
        assert sorted_by_key == sorted_by_cmp


class TestVersionSortkey:
    def test_equal(self):
        assert version_sortkey("1.0") == version_sortkey("1.0")

    def test_ordering(self):
        assert version_sortkey("1.0") < version_sortkey("2.0")

    def test_tilde(self):
        assert version_sortkey("1.0~rc1") < version_sortkey("1.0")

    def test_caret(self):
        assert version_sortkey("1.0") < version_sortkey("1.0^")

    def test_leading_zeros(self):
        assert version_sortkey("10.0001") == version_sortkey("10.1")

    def test_non_alphanumeric_equivalence(self):
        assert version_sortkey("4_0") == version_sortkey("4.0")

    def test_non_ascii_ignored(self):
        assert version_sortkey("1.1.Á.1") == version_sortkey("1.1.1")
