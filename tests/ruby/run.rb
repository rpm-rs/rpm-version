Dir[File.join(__dir__, "test_*.rb")].sort.each { |test| require test }
