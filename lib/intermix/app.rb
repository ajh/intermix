require 'shellwords'
require 'pty'
require 'terminfo'

# This app is the terminal emulator / multiplexer / shell that I am building.
module Intermix
  class App
    attr_accessor :window, :programs, :panes

    def self.logger
      Logger.new 'log/b.log'
    end

    def self.code_to_s(c)
      "#{[c].pack('U').inspect}(0x%02X)" % c
    end

    def initialize
      self.window = Window.new STDOUT

      self.programs = []

      left_pane = Pane.new(window.rows,   window.cols / 2,  0,  0)
      right_pane = Pane.new(window.rows,  window.cols / 2,  0,  window.cols / 2)
      self.panes = [left_pane, right_pane]
    end

    def run(command, pane)
      program = Program.new pane.size_rows, pane.size_cols
      program.run command

      pane.program = program
      self.programs << program
    end

    def main_loop
      window.start

      loop do
        begin
          # TODO: always writes to first program
          programs.first.input << STDIN.read_nonblock(100)
        rescue IO::WaitReadable
        end

        programs.each do |program|
          begin
            program.update
          rescue EOFError
            break
          end
        end

        panes.each do |pane|
          pane.program or next
          window.paint pane.program.screen, pane.offset_row, pane.offset_col
        end
      end

      window.close
    end
  end
end
