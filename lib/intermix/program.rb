require 'pty'
require 'terminfo'

module Intermix

  # This class represents a running program inside the terminal emulator
  class Program
    attr_accessor :parser, :screen, :input, :output, :pid, :size_rows, :size_cols

    def self.logger
      Logger.new 'log/program.log'
    end

    def self.code_to_s(c)
      "#{[c].pack('U').inspect}(0x%02X)" % c
    end

    def initialize(size_rows, size_cols)
      self.parser = TtyOutputParser.new
      self.size_rows = size_rows
      self.size_cols = size_cols
    end

    # Run the program
    def run(command)
      self.output, self.input, self.pid = PTY.spawn command
      TermInfo.tiocswinsz output, size_rows, size_cols

      self.screen = Screen.new size_rows, size_cols, TermInfo.new(ENV['TERM'], output)
      hook_up_parser_to_screen parser, screen

      true
    end

    # Run this often to get the latest update from the program.
    #
    # This'll update the video terminal via the configured parser callbakcs.
    def update
      pid.present? or raise "can't update when program is not running"
      parser << output.read_nonblock(100)

    rescue IO::WaitReadable
      self.class.logger.warn $!.inspect
    end

    private

    def hook_up_parser_to_screen(parser, screen)
      # Define callbacks to hook up the parser to the video terminal
      parser.configure_callbacks do |config|
        config.handle_print do |code|
          screen.print code
        end
        config.handle_csi_dispatch do |sequence|
          self.class.logger.debug "csi_dispatch sequence: #{sequence.inspect}"
          sequence or next

          if sequence.capname == 'sgr' && sequence.param_name == 'normal'
            screen.pen.bold = false
          elsif sequence.capname == 'bold'
            screen.pen.bold = true
          elsif sequence.capname == 'smcup'
            screen.save_cursor
            screen.alternate_screen_buffer
          elsif sequence.capname == 'rmcup'
            screen.save_cursor
            screen.normal_screen_buffer
          end
        end
        config.handle_esc_dispatch do |sequence|
          self.class.logger.debug "esc_dispatch sequence: #{sequence.inspect}"
        end
        config.handle_execute do |sequence|
          self.class.logger.debug "execute_dispatch sequence: #{sequence.inspect}"
          sequence or next

          if sequence.capname == 'cr'
            screen.pen.move_left full: true
          end
          if sequence.capname == 'nl'
            screen.pen.move_down
          end
        end
      end
    end
  end
end
