require "minitest/autorun"
require "rpm_version"

class TestRequirement < Minitest::Test
  def test_no_constraint
    requirement = RPMVersion::Requirement.new("foo")
    assert requirement.satisfies?("foo", RPMVersion::Evr.parse("1.0-1"))
    assert requirement.satisfies?("foo", RPMVersion::Evr.parse("999.0-1"))
    refute requirement.satisfies?("bar", RPMVersion::Evr.parse("1.0-1"))
  end

  def test_equal_constraint
    requirement = RPMVersion::Requirement.new("foo", "=", RPMVersion::Evr.parse("1.0-1"))
    assert requirement.satisfies?("foo", RPMVersion::Evr.parse("1.0-1"))
    refute requirement.satisfies?("foo", RPMVersion::Evr.parse("2.0-1"))
  end

  def test_greater_than_or_equal_constraint
    requirement = RPMVersion::Requirement.new("foo", ">=", RPMVersion::Evr.parse("1.0-1"))
    assert requirement.satisfies?("foo", RPMVersion::Evr.parse("1.0-1"))
    assert requirement.satisfies?("foo", RPMVersion::Evr.parse("2.0-1"))
    refute requirement.satisfies?("foo", RPMVersion::Evr.parse("0.9-1"))
  end

  def test_less_than_constraint
    requirement = RPMVersion::Requirement.new("foo", "<", RPMVersion::Evr.parse("2.0-1"))
    assert requirement.satisfies?("foo", RPMVersion::Evr.parse("1.0-1"))
    refute requirement.satisfies?("foo", RPMVersion::Evr.parse("2.0-1"))
    refute requirement.satisfies?("foo", RPMVersion::Evr.parse("3.0-1"))
  end

  def test_string_and_inspect
    assert_equal "foo", RPMVersion::Requirement.new("foo").to_s
    requirement = RPMVersion::Requirement.new("foo", ">=", RPMVersion::Evr.parse("1:2.0-1"))
    assert_equal "foo >= 1:2.0-1", requirement.to_s
    assert_includes requirement.inspect, "Requirement"
  end

  def test_name_and_constraint
    requirement = RPMVersion::Requirement.new("foo", ">=", RPMVersion::Evr.parse("1.0-1"))
    assert_equal "foo", requirement.name
    assert_equal [">=", RPMVersion::Evr.parse("1.0-1")], requirement.constraint
  end

  def test_invalid_operator
    assert_raises(ArgumentError) do
      RPMVersion::Requirement.new("foo", "!=", RPMVersion::Evr.parse("1.0-1"))
    end
  end

  def test_incomplete_constraint
    assert_raises(ArgumentError) { RPMVersion::Requirement.new("foo", ">=") }
  end
end
