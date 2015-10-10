#!/usr/bin/env ruby

def lint_infocmp_output(lines)
  lines.map do |line|
    line =~ %r/=/ or next
    line = line.sub %r/^\s+/, ''
    line = line.sub %r/,$/, ''
  end.compact
end

class Capability
  attr_accessor :capname
  attr_accessor :long_name
  attr_accessor :sequence

  def initialize(options={})
    options.each {|m, v|
      send "#{m}=", v
    }
  end

  def inspect
    super.gsub %r/\s+/, "\t"
  end
end

capabilities = []

lint_infocmp_output(`infocmp -l -1`.lines).map do |cap|
  match = %r/^(\w+)=(.+)$/.match cap
  capname = match[1]
  sequence = match[2]
  capabilities << Capability.new(capname: capname, sequence: sequence)
end

lint_infocmp_output(`infocmp -L -1`.lines).map do |cap|
  match = %r/^(\w+)=(.+)$/.match cap
  long_name = match[1]
  sequence = match[2]
  capability = capabilities.find {|c| c.sequence == sequence}
  capability.long_name = long_name if capability
end

capabilities.each {|c| puts "#{c.capname}\t#{c.long_name}\t#{c.sequence}" }
