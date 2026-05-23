from rpm_version import Evr, Nevra, Requirement


class TestRequirement:
    def test_no_constraint(self):
        req = Requirement("foo")
        assert req.satisfies("foo", Evr.parse("1.0-1"))
        assert req.satisfies("foo", Evr.parse("999.0-1"))
        assert not req.satisfies("bar", Evr.parse("1.0-1"))

    def test_eq(self):
        req = Requirement("foo", "=", Evr.parse("1.0-1"))
        assert req.satisfies("foo", Evr.parse("1.0-1"))
        assert not req.satisfies("foo", Evr.parse("2.0-1"))

    def test_ge(self):
        req = Requirement("foo", ">=", Evr.parse("1.0-1"))
        assert req.satisfies("foo", Evr.parse("1.0-1"))
        assert req.satisfies("foo", Evr.parse("2.0-1"))
        assert not req.satisfies("foo", Evr.parse("0.9-1"))

    def test_lt(self):
        req = Requirement("foo", "<", Evr.parse("2.0-1"))
        assert req.satisfies("foo", Evr.parse("1.0-1"))
        assert not req.satisfies("foo", Evr.parse("2.0-1"))
        assert not req.satisfies("foo", Evr.parse("3.0-1"))

    def test_str(self):
        assert str(Requirement("foo")) == "foo"
        assert str(Requirement("foo", ">=", Evr.parse("1:2.0-1"))) == "foo >= 1:2.0-1"

    def test_repr(self):
        req = Requirement("foo", ">=", Evr.parse("2.0-1"))
        assert repr(req) == "Requirement(foo >= 2.0-1)"

    def test_name(self):
        req = Requirement("foo", "=", Evr.parse("1.0-1"))
        assert req.name == "foo"

    def test_invalid_op(self):
        import pytest

        with pytest.raises(ValueError):
            Requirement("foo", "!=", Evr.parse("1.0-1"))

    def test_partial_args(self):
        import pytest

        with pytest.raises(ValueError):
            Requirement("foo", ">=")
