require "rubygems"

cargo_toml = File.read(File.join(__dir__, "Cargo.toml"))
version = cargo_toml.match(/^version = "([^"]+)"$/)[1]

Gem::Specification.new do |spec|
  spec.name = "rpm-version"
  spec.version = version
  spec.authors = ["Daniel Alley"]
  spec.email = ["dalley@redhat.com"]
  spec.summary = "RPM-compatible version parsing and comparison"
  spec.description = "Ruby bindings for RPM EVR and NEVRA parsing, comparison, and sorting."
  spec.homepage = "https://github.com/rpm-rs/rpm-version"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 3.3"

  spec.metadata["source_code_uri"] = "https://github.com/rpm-rs/rpm-version"
  spec.metadata["changelog_uri"] = "https://github.com/rpm-rs/rpm-version/blob/master/CHANGELOG.md"

  spec.files = [
    "Cargo.lock",
    "Cargo.toml",
    "LICENSE",
    "RUBY_README.md",
    "benches/sort.rs",
    "examples/compare_versions.rs",
    "examples/serde.rs",
    "examples/sortkey.rs",
    "ruby/ext/rpm_version/extconf.rb",
  ] + Dir.glob(["ruby/lib/**/*.rb", "src/**/*.rs"])
  spec.files.sort!
  spec.require_paths = ["ruby/lib"]
  spec.extensions = ["ruby/ext/rpm_version/extconf.rb"]

  spec.add_dependency "rb_sys", ">= 0.9.128", "< 1.0"
end
