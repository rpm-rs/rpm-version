require "minitest/autorun"
require "rpm_version"

class TestEvr < Minitest::Test
  def test_construct
    evr = RPMVersion::Evr.new("1", "2.3.4", "5.el9")
    assert_equal "1", evr.epoch
    assert_equal "2.3.4", evr.version
    assert_equal "5.el9", evr.release
  end

  def test_parse
    evr = RPMVersion::Evr.parse("1:2.3.4-5")
    assert_equal "1", evr.epoch
    assert_equal "2.3.4", evr.version
    assert_equal "5", evr.release
  end

  def test_parse_without_epoch
    evr = RPMVersion::Evr.parse("2.3.4-5")
    assert_equal "", evr.epoch
    assert_equal "2.3.4", evr.version
    assert_equal "5", evr.release
  end

  def test_string_forms
    evr = RPMVersion::Evr.new("1", "2.3", "4")
    assert_equal "1:2.3-4", evr.to_s
    assert_equal "1:2.3-4", evr.evr
    assert_equal "1:2.3-4", evr.evr_short
  end

  def test_inspect
    assert_includes RPMVersion::Evr.parse("1.0-1").inspect, "Evr"
  end

  def test_ordering
    first = RPMVersion::Evr.parse("1.0-1")
    second = RPMVersion::Evr.parse("2.0-1")
    assert_operator first, :<, second
    assert_operator second, :>, first
    assert_operator first, :<=, second
    assert_operator second, :>=, first
  end

  def test_equality
    assert_equal RPMVersion::Evr.parse("1.0-1"), RPMVersion::Evr.parse("1.0-1")
    refute_equal RPMVersion::Evr.parse("1.0-1"), RPMVersion::Evr.parse("2.0-1")
  end

  def test_epoch_comparison
    assert_operator RPMVersion::Evr.parse("1:1.0-1"), :<, RPMVersion::Evr.parse("2:1.0-1")
  end

  def test_tilde_and_caret_ordering
    assert_operator RPMVersion::Evr.parse("1.0~rc1-1"), :<, RPMVersion::Evr.parse("1.0-1")
    assert_operator RPMVersion::Evr.parse("1.0~rc1-1"), :<, RPMVersion::Evr.parse("1.0~rc2-1")
    assert_operator RPMVersion::Evr.parse("1.0-1"), :<, RPMVersion::Evr.parse("1.0^git1-1")
    assert_operator RPMVersion::Evr.parse("1.0^git1-1"), :<, RPMVersion::Evr.parse("1.1-1")
  end
end

class TestEvrCompare < Minitest::Test
  def test_less
    assert_equal(-1, RPMVersion.evr_compare("1.0-1", "2.0-1"))
  end

  def test_greater
    assert_equal 1, RPMVersion.evr_compare("2:1.0-1", "1:9.9-1")
  end

  def test_equal
    assert_equal 0, RPMVersion.evr_compare("1.0-1", "1.0-1")
  end

  def test_release_comparison
    assert_equal(-1, RPMVersion.evr_compare("1.0-1", "1.0-2"))
  end
end

class TestNevra < Minitest::Test
  def test_construct_and_parse
    nevra = RPMVersion::Nevra.new("foo", "1", "2.3", "4", "x86_64")
    parsed = RPMVersion::Nevra.parse("foo-1:2.3-4.x86_64")
    assert_equal "foo", parsed.name
    assert_equal "1", parsed.epoch
    assert_equal "2.3", parsed.version
    assert_equal "4", parsed.release
    assert_equal "x86_64", parsed.arch
    assert_equal nevra, parsed
  end

  def test_string_forms
    nevra = RPMVersion::Nevra.parse("foo-1:2.3-4.x86_64")
    assert_equal "foo-1:2.3-4.x86_64", nevra.to_s
    assert_equal "foo-1:2.3-4.x86_64", nevra.nevra
    assert_equal "foo-1:2.3-4.x86_64", nevra.nevra_short
    assert_equal "foo-2.3-4.x86_64", nevra.nvra
  end

  def test_evr
    evr = RPMVersion::Nevra.parse("foo-1:2.3-4.x86_64").evr
    assert_instance_of RPMVersion::Evr, evr
    assert_equal "1", evr.epoch
    assert_equal "2.3", evr.version
  end

  def test_ordering
    first = RPMVersion::Nevra.parse("foo-1.0-1.x86_64")
    second = RPMVersion::Nevra.parse("foo-2.0-1.x86_64")
    assert_operator first, :<, second
  end

  def test_equality
    first = RPMVersion::Nevra.parse("foo-1.0-1.x86_64")
    second = RPMVersion::Nevra.parse("foo-1.0-1.x86_64")
    assert_equal first, second
  end
end

class TestEvrSort < Minitest::Test
  def test_basic_sort
    assert_equal ["1.0-1", "2.0-1", "3.0-1"],
      RPMVersion.evr_sort(["2.0-1", "1.0-1", "3.0-1"])
  end

  def test_epoch_sort
    assert_equal ["1.0-1", "2.0-1", "1:0.5-1"],
      RPMVersion.evr_sort(["2.0-1", "1:0.5-1", "1.0-1"])
  end

  def test_tilde_and_caret
    sorted = ["1.0~rc1-1", "1.0-1", "1.0^git1-1", "1.1-1"]
    versions = ["1.0^git1-1", "1.0-1", "1.0~rc1-1", "1.1-1"]
    assert_equal sorted, RPMVersion.evr_sort(versions)
  end

  def test_empty_and_singleton
    assert_equal [], RPMVersion.evr_sort([])
    assert_equal ["1.0-1"], RPMVersion.evr_sort(["1.0-1"])
  end
end

class TestNevraSort < Minitest::Test
  def test_basic_sort
    input = ["foo-2.0-1.x86_64", "bar-1.0-1.x86_64", "foo-1.0-1.x86_64"]
    expected = ["bar-1.0-1.x86_64", "foo-1.0-1.x86_64", "foo-2.0-1.x86_64"]
    assert_equal expected, RPMVersion.nevra_sort(input)
  end

  def test_architecture_sort
    input = ["foo-1.0-1.x86_64", "foo-1.0-1.i686", "foo-1.0-1.noarch"]
    expected = ["foo-1.0-1.i686", "foo-1.0-1.noarch", "foo-1.0-1.x86_64"]
    assert_equal expected, RPMVersion.nevra_sort(input)
  end

  def test_empty
    assert_equal [], RPMVersion.nevra_sort([])
  end
end

class TestHash < Minitest::Test
  def test_evr_hash_consistency
    values = { RPMVersion::Evr.parse("1.0-1") => :found }
    assert_equal :found, values[RPMVersion::Evr.parse("0:1.0-1")]
  end

  def test_nevra_hash_consistency
    values = { RPMVersion::Nevra.parse("foo-1.0-1.x86_64") => :found }
    assert_equal :found, values[RPMVersion::Nevra.parse("foo-0:1.0-1.x86_64")]
  end
end
