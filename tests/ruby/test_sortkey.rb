require "minitest/autorun"
require "rpm_version"

class TestEvrSortKey < Minitest::Test
  def test_from_values_and_parse
    parsed = RPMVersion::EvrSortKey.parse("2:1.0~rc1-3.fc40")
    values = RPMVersion::EvrSortKey.from_values("2", "1.0~rc1", "3.fc40")
    assert_equal values, parsed
    assert_equal Encoding::BINARY, parsed.encoding
  end

  def test_epoch_ordering
    no_epoch = RPMVersion::EvrSortKey.from_values(nil, "1.2.3", "45")
    zero_epoch = RPMVersion::EvrSortKey.from_values("0", "1.2.3", "45")
    one_epoch = RPMVersion::EvrSortKey.from_values("1", "1.2.3", "45")
    assert_equal no_epoch, zero_epoch
    assert_operator no_epoch, :<, one_epoch
  end

  def test_version_release_and_special_ordering
    assert_operator RPMVersion::EvrSortKey.from_values("", "1.2.3", "45"), :<,
      RPMVersion::EvrSortKey.from_values("", "1.2.4", "45")
    assert_operator RPMVersion::EvrSortKey.from_values("", "1.2.3", "3"), :<,
      RPMVersion::EvrSortKey.from_values("", "1.2.3", "10")
    assert_operator RPMVersion::EvrSortKey.parse("1.0~rc1-1"), :<,
      RPMVersion::EvrSortKey.parse("1.0-1")
    assert_operator RPMVersion::EvrSortKey.parse("1.0-1"), :<,
      RPMVersion::EvrSortKey.parse("1.0^git1-1")
  end
end

class TestEvrSortKeyMethod < Minitest::Test
  def test_matches_sortkey_module
    evr = RPMVersion::Evr.parse("2:1.0~rc1-3.fc40")
    assert_equal evr.sortkey, RPMVersion::EvrSortKey.from_values("2", "1.0~rc1", "3.fc40")
  end

  def test_sortkey_order_matches_evr_order
    pairs = [["1.0-1", "2.0-1"], ["1.0~rc1-1", "1.0-1"], ["1.0-1", "1.0^git1-1"]]
    pairs.each do |first, second|
      first_evr = RPMVersion::Evr.parse(first)
      second_evr = RPMVersion::Evr.parse(second)
      assert_equal first_evr < second_evr, first_evr.sortkey < second_evr.sortkey
    end
  end
end

class TestVersionSortKey < Minitest::Test
  def test_ordering
    assert_operator RPMVersion.version_sortkey("1.0"), :<, RPMVersion.version_sortkey("2.0")
    assert_operator RPMVersion.version_sortkey("1.0~rc1"), :<,
      RPMVersion.version_sortkey("1.0")
  end

  def test_equivalent_versions
    assert_equal RPMVersion.version_sortkey("10.0001"), RPMVersion.version_sortkey("10.1")
    assert_equal RPMVersion.version_sortkey("4_0"), RPMVersion.version_sortkey("4.0")
  end
end
