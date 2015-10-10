#!/usr/bin/env ruby

require 'terminfo'

# A test app
module A
  class App
    attr_accessor :terminfo

    def initialize
      self.terminfo = TermInfo.new ENV['TERM'], STDOUT
    end

    def run
      terminfo.control 'smcup'

      terminfo.control 'cup', 1, 5
      terminfo.control 'bold'
      print 'I am bold'

      terminfo.control 'cup', 2, 7
      terminfo.control 'sgr0'
      print 'I am normal'

      terminfo.control 'cup', 5, 5
      print 'bye'

      gets

      terminfo.control 'rmcup'
    end
  end
end

A::App.new.run
