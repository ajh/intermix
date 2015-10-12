require 'shellwords'
require 'pty'
require 'terminfo'
require 'io/console'

# This app is the terminal emulator / multiplexer / shell that I am building.
module Intermix
  class App
    attr_accessor :window, :programs, :panes, :mode, :status_pane

    def self.logger
      @logger ||= Logger.new 'log/app.log'
    end

    def self.code_to_s(c)
      "#{[c].pack('U').inspect}(0x%02X)" % c
    end

    def initialize
      self.window = Window.new STDOUT

      self.programs = []

      #left_pane   = Pane.new(window.rows - 1,   window.cols / 2,  0,  0, "left")
      #right_pane  = Pane.new(window.rows - 1,  window.cols / 2,  0,  window.cols / 2, "right")
      self.status_pane = StatusPane.new(window)
      #self.panes = [left_pane, right_pane, status_pane]
      self.panes = [status_pane]

      self.mode = CommandMode.new(self)
      self.mode.enter
    end

    def run(command, pane)
      program = Program.new pane.size_rows, pane.size_cols, command
      program.run

      pane.screen = program.screen
      pane.program = program

      self.programs << program
    end

    def main_loop
      STDIN.raw!
      window.start

      loop do
        begin
          #programs.first.input << STDIN.read_nonblock(100)

          STDIN.read_nonblock(100).each_char do |char|
            mode.handle_key char
          end
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
          pane.screen or next
          window.paint pane.screen, pane.offset_row, pane.offset_col, @refresh
          @refresh = false
        end
      end

    ensure
      STDIN.cooked! # this assumes we were cooked at the start
      window.close rescue nil
    end

    def refresh
      @refresh = true
    end

    def status=(msg)
      status_pane.status = "#{mode.class.to_s.demodulize.underscore.humanize}: #{msg}"
    end
  end
end
