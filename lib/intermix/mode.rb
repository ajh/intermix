require 'active_support/all'

module Intermix

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
      if status_pane = app.panes.find {|p| p.name == "status" }
        status_pane.status = self.class.to_s.demodulize.underscore.humanize
      end
    end

    def handle_key(key)
      if ["\x03", "\x1C"].include? key
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
      self.selected_pane = 0
    end

    def handle_key(key)
      super key

      if (1..9).to_a.map(&:to_s).include? key
        self.selected_pane = key.to_i - 1
        self.class.logger.info "selected pane: #{selected_pane}"
      elsif key == 'i'
        switch_mode ProgramMode.new(app, app.panes[selected_pane].program)
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

      if "\c-b" == key
        switch_mode CommandMode.new(app)
      else
        program.input << key
      end
    end
  end
end
