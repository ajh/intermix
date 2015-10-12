require 'active_support/all'
require 'pry'
require 'pry-nav'

module Intermix
  # found expirementaly by doing:
  #
  #   irb -r io/console
  #   $ STDIN.getch
  #
  KEYS = {
    "ctrl-c" => "\x3",
    "ctrl-p" => "\x10",
    "ctrl-b" => "\x2",
  }.freeze

  # Base class for modes
  class Mode
    attr_accessor :app

    def self.logger
      Logger.new 'log/mode.log'
    end

    def initialize(app)
      self.app = app
    end

    def enter
      self.class.logger.info "enter #{self}"
      app.status = "enter"
    end

    def handle_key(key)
      if KEYS['ctrl-c'] == key
        Kernel.exit
      end
    end

    def exit
      self.class.logger.info "exit #{self}"
    end

    def switch_mode(new_mode)
      self.class.logger.info "switching mode from:#{self} to:#{new_mode}"
      self.exit
      app.mode = new_mode
      new_mode.enter
    end
  end

  class CommandMode < Mode
    attr_accessor :selected_pane

    def self.logger; Mode.logger; end

    def initialize(*args)
      super(*args)
      self.selected_pane = app.panes.first
    end

    def handle_key(key)
      super key

      if (0..9).to_a.map(&:to_s).include? key
        if pane = app.panes[key.to_i]
          self.selected_pane = pane
          app.status = "selected pane: #{pane.inspect}"
        end
      elsif key == 'i'
        if selected_pane
          switch_mode ProgramMode.new(app, selected_pane.program)
        end
      elsif KEYS['ctrl-p'] == key
        switch_mode PryMode.new(app)
      end
    end
  end

  class ProgramMode < Mode
    attr_accessor :program

    def initialize(*args, program)
      super(*args)
      self.program = program
    end

    def handle_key(key)
      super key

      if KEYS['ctrl-b'] == key
        switch_mode CommandMode.new(app)
      else
        if program.state == :running
          begin
            program.input << key

          rescue EOFError, Errno::EIO
            # this is not my responsiblity, but oh well
            program.kill
          end
        end
      end
    end
  end

  class PryMode < Mode
    def enter
      super

      STDIN.cooked!
      self.pry
      STDIN.raw!

      switch_mode CommandMode.new(app)
    end

    def split_and_run(command)
      new_pane = Pane.new(0, app.window.cols, 0, 0, command)
      app.panes << new_pane

      resize_panes

      app.run command, new_pane
      app.refresh

      true
    end

    def panes
      app.panes.each_with_index do |p, i|
        puts "#{i}: #{p.name} #{p.inspect}"
      end

      nil
    end

    def close_pane(index)
      pane = app.panes[index]
      pane.program.kill
      app.panes -= [pane]

      resize_panes
    end

    private

    def resize_panes
      panes_to_size = app.panes.reject {|p| p.name == 'status'}

      size_of_each_pane = if panes_to_size.length > 1
                            app.window.rows / panes_to_size.length
                          else
                            app.window.rows
                          end

      panes_to_size.each_with_index do |p, i|
        p.size_rows = size_of_each_pane
        p.offset_row = size_of_each_pane * i
      end
    end
  end
end
