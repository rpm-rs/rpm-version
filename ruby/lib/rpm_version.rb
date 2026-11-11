require "rpm_version/rpm_version"

module RPMVersion
  class Requirement
    class << self
      alias_method :__native_new, :new

      def new(name, op = nil, evr = nil)
        __native_new(name, op, evr)
      end
    end
  end
end
