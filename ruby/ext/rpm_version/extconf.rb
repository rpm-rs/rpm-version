require "mkmf"
require "rb_sys/mkmf"

create_rust_makefile("rpm_version/rpm_version") do |config|
  config.ext_dir = "./../../.."
  config.features = ["ruby"]
  config.profile = :release
end
