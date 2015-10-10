require 'set'
require 'active_support/all'
require 'pp'

module Intermix

  # See http://www.vt100.net/emu/dec_ansi_parser
  #
  # This is a parser that uses a state machine and fires off callbacks based on incoming characters
  class TtyOutputParser
    attr_accessor :callback_config
    attr_accessor :state_machine

    attr_accessor :private_flag, :intermediate_characters, :final_character, :parameters

    # Utilities and such
    class << self
      def logger
        Logger.new('log/parser.log').tap do |l|
          l.sev_threshold = Logger::WARN
        end
      end

      def build_set(*ranges_and_values)
        set = Set.new ranges_and_values.map{|e| e.is_a?(Range) ? e.to_a : e }.flatten

        # A0-FF range is equivalent to 20-7F range
        equiv_set = Set.new
        set.each do |byte|
          if byte >= 0xa0
            equiv_set << byte - 0x80
          elsif (0x20..0x7f).include? byte
            equiv_set << byte + 0x80
          end
        end

        set + equiv_set
      end

      def bytes_to_characters(bytes)
        bytes.map{|b| byte_to_s b}
      end

      def byte_to_s(byte)
        [byte].pack 'U'
      end
    end

    def initialize
      self.callback_config = CallbackConfig.new

      self.intermediate_characters = []
      self.parameters = []
      self.private_flag = false
    end

    # Add a string of codes to the parser. Not sure if multibyte characters are
    # correctly handled here.
    def <<(string)
      string.each_byte { |byte| state_machine.byte = byte }
    end

    ACTIONS = %w(
      ignore
      print
      execute
      clear
      collect
      param
      esc_dispatch
      csi_dispatch
      hook
      put
      unhook
      osc_start
      osc_put
      osc_end
    ).map(&:to_sym).freeze

    # The character or control is not processed. No observable difference in the
    # terminal’s state would occur if the character that caused this action was
    # not present in the input stream. (Therefore, this action can only occur
    # within a state.)
    def ignore(byte); end

    # This action only occurs in ground state. The current code should be mapped
    # to a glyph according to the character set mappings and shift states in
    # effect, and that glyph should be displayed. 20 (SP) and 7F (DEL) have
    # special behaviour in later VT series, as described in ground.
    def print(byte)
      callback_config.print.try :call, byte
    end

    # The C0 or C1 control function should be executed, which may have any one of
    # a variety of effects, including changing the cursor position, suspending or
    # resuming communications or changing the shift states in effect. There are
    # no parameters to this action.
    def execute(byte)
      s = TtyCodes.find_control codes: [byte]

      unless s
        self.class.logger.warn "unimplmented execute: #{self.class.byte_to_s(byte).inspect}"
      end

      callback_config.execute or return
      callback_config.execute.call s
    end

    # This action causes the current private flag, intermediate characters, final
    # character and parameters to be forgotten. This occurs on entry to the
    # escape, csi entry and dcs entry states, so that erroneous sequences like
    # CSI 3 ; 1 CSI 2 J are handled correctly.
    def clear(byte=nil)
      self.private_flag = false
      self.intermediate_characters = []
      self.final_character = nil
      self.parameters = []
    end

    # The private marker or intermediate character should be stored for later use
    # in selecting a control function to be executed when a final character
    # arrives. X3.64 doesn’t place any limit on the number of intermediate
    # characters allowed before a final character, although it doesn’t define any
    # control sequences with more than one. Digital defined escape sequences with
    # two intermediate characters, and control sequences and device control
    # strings with one. If more than two intermediate characters arrive, the
    # parser can just flag this so that the dispatch can be turned into a null
    # operation.
    def collect(byte)
      self.intermediate_characters << byte
    end

    # This action collects the characters of a parameter string for a control
    # sequence or device control sequence and builds a list of parameters. The
    # characters processed by this action are the digits 0-9 (codes 30-39) and
    # the semicolon (code 3B). The semicolon separates parameters. There is no
    # limit to the number of characters in a parameter string, although a maximum
    # of 16 parameters need be stored. If more than 16 parameters arrive, all the
    # extra parameters are silently ignored.
    #
    # The VT500 Programmer Information is inconsistent regarding the maximum
    # value that a parameter can take. In section 4.3.3.2 of EK-VT520-RM it says
    # that “any parameter greater than 9999 (decimal) is set to 9999 (decimal)”.
    # However, in the description of DECSR (Secure Reset), its parameter is
    # allowed to range from 0 to 16383. Because individual control functions need
    # to make sure that numeric parameters are within specific limits, the
    # supported maximum is not critical, but it must be at least 16383.
    #
    # Most control functions support default values for their parameters. The
    # default value for a parameter is given by either leaving the parameter
    # blank, or specifying a value of zero. Judging by previous threads on the
    # newsgroup comp.terminals, this causes some confusion, with the occasional
    # assertion that zero is the default parameter value for control functions.
    # This is not the case: many control functions have a default value of 1, one
    # (GSM) has a default value of 100, and some have no default. However, in all
    # cases the default value is represented by either zero or a blank value.
    #
    # In the standard ECMA-48, which can be considered X3.64’s successor², there
    # is a distinction between a parameter with an empty value (representing the
    # default value), and one that has the value zero. There used to be a mode,
    # ZDM (Zero Default Mode), in which the two cases were treated identically,
    # but that is now deprecated in the fifth edition (1991). Although a VT500
    # parser needs to treat both empty and zero parameters as representing the
    # default, it is worth considering future extensions by distinguishing them
    # internally.
    def param(byte)
      self.parameters << byte
    end

    # The final character of an escape sequence has arrived, so determined the
    # control function to be executed from the intermediate character(s) and
    # final character, and execute it. The intermediate characters are available
    # because collect stored them as they arrived.
    def esc_dispatch(byte)
      s = TtyCodes.find_escape codes: [byte]

      unless s
        self.class.logger.warn "unimplmented esc sequence: #{self.class.byte_to_s(byte).inspect}"
      end

      callback_config.esc_dispatch or return
      callback_config.esc_dispatch.call s
    end

    # A final character has arrived, so determine the control function to be
    # executed from private marker, intermediate character(s) and final
    # character, and execute it, passing in the parameter list. The private
    # marker and intermediate characters are available because collect stored
    # them as they arrived.
    #
    # Digital mostly used private markers to extend the parameters of existing
    # X3.64-defined control functions, while keeping a similar meaning. A few
    # examples are shown in the table below.
    #
    # No Private MarkerWith Private Marker SM, Set ANSI ModesSM, Set Digital
    # Private Modes ED, Erase in DisplayDECSED, Selective Erase in Display CPR,
    # Cursor Position ReportDECXCPR, Extended Cursor Position Report
    #
    # In the cases above, csi_dispatch needn’t know about the private marker at
    # all, as long as it is passed along to the control function when it is
    # executed. However, the VT500 has a single case where the use of a private
    # marker selects an entirely different control function (DECSTBM, Set Top and
    # Bottom Margins and DECPCTERM, Enter/Exit PCTerm or Scancode Mode), so this
    # action needs to use the private marker in its choice. xterm takes the same
    # approach for efficiency, even though it doesn’t support DECPCTERM.
    #
    # The selected control function will have access to the list of parameters,
    # which it will use some or all of. If more parameters are supplied than the
    # control function requires, only the earliest parameters will be used and
    # the rest will be ignored. If too few parameters are supplied, default
    # values will be used. If the control function has no default values,
    # defaulted parameters will be ignored; this may result in the control
    # function having no effect. For example, if the SM (Set Mode) control
    # function is invoked with the sequence CSI 2;0;5 h, the second parameter
    # will be ignored because SM has no default value.
    def csi_dispatch(byte)
      self.intermediate_characters << byte

      param_chars = self.class.bytes_to_characters(parameters).join.split ';'

      s = TtyCodes.find_csi \
        codes: intermediate_characters,
        param_chars: param_chars

      unless s
        i = self.class.bytes_to_characters intermediate_characters
        p = self.class.bytes_to_characters parameters
        self.class.logger.warn \
          "unimplmented csi sequence intermediate_characters:#{i.inspect} parameters:#{p.inspect}"
      end

      callback_config.csi_dispatch or return
      callback_config.csi_dispatch.call s
    end

    # This action is invoked when a final character arrives in the first part of
    # a device control string. It determines the control function from the
    # private marker, intermediate character(s) and final character, and executes
    # it, passing in the parameter list. It also selects a handler function for
    # the rest of the characters in the control string. This handler function
    # will be called by the put action for every character in the control string
    # as it arrives.
    #
    # This way of handling device control strings has been selected because it
    # allows the simple plugging-in of extra parsers as functionality is added.
    # Support for a fairly simple control string like DECDLD (Downline Load)
    # could be added into the main parser if soft characters were required, but
    # the main parser is no place for complicated protocols like ReGIS.
    def hook(byte)
      self.class.logger.warn "unimplemented method #{__method__}"
    end

    # This action passes characters from the data string part of a device control
    # string to a handler that has previously been selected by the hook action.
    # C0 controls are also passed to the handler.
    def put(byte)
      self.class.logger.warn "unimplemented method #{__method__}"
    end

    # When a device control string is terminated by ST, CAN, SUB or ESC, this
    # action calls the previously selected handler function with an “end of data”
    # parameter. This allows the handler to finish neatly.
    def unhook(byte)
      self.class.logger.warn "unimplemented method #{__method__}"
    end

    # When the control function OSC (Operating System Command) is recognised,
    # this action initializes an external parser (the “OSC Handler”) to handle
    # the characters from the control string. OSC control strings are not
    # structured in the same way as device control strings, so there is no choice
    # of parsers.
    def osc_start(byte)
      self.class.logger.warn "unimplemented method #{__method__}"
    end

    # This action passes characters from the control string to the OSC Handler as
    # they arrive. There is therefore no need to buffer characters until the end
    # of the control string is recognised.
    def osc_put(byte)
      self.class.logger.warn "unimplemented method #{__method__}"
    end

    # This action is called when the OSC string is terminated by ST, CAN, SUB or
    # ESC, to allow the OSC handler to finish neatly.
    def osc_end(byte)
      self.class.logger.warn "unimplemented method #{__method__}"
    end

    # define callbacks to parser events. Use like this:
    #
    #   def handle_some_event(code)
    #   end
    #
    #   p = self.class.new
    #   p.configure_callbacks do |config|
    #     config.print do |code|
    #     end
    #
    #     config.other &handle_some_event
    #   end
    #
    def configure_callbacks
      yield callback_config
      true
    end

    private

    class CallbackConfig
      attr_accessor(*TtyOutputParser::ACTIONS)

      TtyOutputParser::ACTIONS.each do |action|
        class_eval <<-RUBY, __FILE__, __LINE__ + 1
          def handle_#{action}(&block)
            self.#{action} = block
          end
        RUBY
      end
    end

    def state_machine
      unless defined? @state_machine
        m = StateMachine.new self, State.subclasses, Ground

        # Note: the transitions with no `from` are at the end because a more
        # specific transition should be found first
        m.define_transition from: CsiEntry, to: CsiIgnore, code_set: TtyOutputParser.build_set(0x3a)
        m.define_transition from: CsiEntry, to: CsiIntermediate, action: :collect, code_set: TtyOutputParser.build_set(0x20..0x2f)
        m.define_transition from: CsiEntry, to: CsiParam, action: :collect, code_set: TtyOutputParser.build_set(0x3c..0x3f)
        m.define_transition from: CsiEntry, to: CsiParam, action: :param, code_set: TtyOutputParser.build_set(0x30..0x39, 0x3b)
        m.define_transition from: CsiEntry, to: Ground, action: :csi_dispatch, code_set: TtyOutputParser.build_set(0x40..0x7e)
        m.define_transition from: CsiIgnore, to: Ground, code_set: TtyOutputParser.build_set(0x30..0x7e)
        m.define_transition from: CsiIntermediate, to: CsiIgnore, code_set: TtyOutputParser.build_set(0x30..0x3f)
        m.define_transition from: CsiIntermediate, to: Ground, action: :csi_dispatch, code_set: TtyOutputParser.build_set(0x40..0x7e)
        m.define_transition from: CsiParam, to: CsiIgnore, code_set: TtyOutputParser.build_set(0x3a, 0x3c..0x3f)
        m.define_transition from: CsiParam, to: CsiIntermediate, action: :collect, code_set: TtyOutputParser.build_set(0x20..0x2f)
        m.define_transition from: CsiParam, to: Ground, action: :csi_dispatch, code_set: TtyOutputParser.build_set(0x40..0x7e)
        m.define_transition from: DcsEntry, to: DcsIgnore, code_set: TtyOutputParser.build_set(0x3a)
        m.define_transition from: DcsEntry, to: DcsIntermediate, action: :collect, code_set: TtyOutputParser.build_set(0x20..0x2f)
        m.define_transition from: DcsEntry, to: DcsParam, action: :collect, code_set: TtyOutputParser.build_set(0x3c..0x3f)
        m.define_transition from: DcsEntry, to: DcsParam, action: :param, code_set: TtyOutputParser.build_set(0x30..0x39, 0x3b)
        m.define_transition from: DcsEntry, to: DcsPassthrough, code_set: TtyOutputParser.build_set(0x30..0x7e)
        m.define_transition from: DcsIgnore, to: Ground, code_set: TtyOutputParser.build_set(0x9c)
        m.define_transition from: DcsIntermediate, to: DcsIgnore, code_set: TtyOutputParser.build_set(0x30..0x3f)
        m.define_transition from: DcsIntermediate, to: DcsPassthrough, code_set: TtyOutputParser.build_set(0x30..0x7e)
        m.define_transition from: DcsParam, to: DcsIgnore, code_set: TtyOutputParser.build_set(0x3a, 0x3c..0x3f)
        m.define_transition from: DcsParam, to: DcsIntermediate, action: :collect, code_set: TtyOutputParser.build_set(0x20..0x2f)
        m.define_transition from: DcsParam, to: DcsPassthrough, code_set: TtyOutputParser.build_set(0x40..0x7e)
        m.define_transition from: DcsPassthrough, to: Ground, code_set: TtyOutputParser.build_set(0x9c)
        m.define_transition from: Escape, to: CsiEntry, code_set: TtyOutputParser.build_set(0x5b)
        m.define_transition from: Escape, to: DcsEntry, code_set: TtyOutputParser.build_set(0x50)
        m.define_transition from: Escape, to: EscapeIntermediate, action: :collect, code_set: TtyOutputParser.build_set(0x20..0x2f)
        m.define_transition from: Escape, to: Ground, action: :esc_dispatch, code_set: TtyOutputParser.build_set(0x30..0x4f, 0x51..0x57, 0x59, 0x5a, 0x5c, 0x60..0x7e)
        m.define_transition from: Escape, to: OscString, code_set: TtyOutputParser.build_set(0x5d)
        m.define_transition from: Escape, to: SosPmApcString, code_set: TtyOutputParser.build_set(0x58, 0x5e, 0x5f)
        m.define_transition from: EscapeIntermediate, to: Ground, action: :esc_dispatch, code_set: TtyOutputParser.build_set(0x30..0x7e)
        m.define_transition from: OscString, to: Ground, code_set: TtyOutputParser.build_set(0x9c)
        m.define_transition from: SosPmApcString, to: Ground, code_set: TtyOutputParser.build_set(0x9c)
        m.define_transition from: nil, to: CsiEntry, code_set: TtyOutputParser.build_set(0x9d)
        m.define_transition from: nil, to: DcsEntry, code_set: TtyOutputParser.build_set(0x90)
        m.define_transition from: nil, to: Escape, code_set: TtyOutputParser.build_set(0x1b)
        m.define_transition from: nil, to: Ground, code_set: TtyOutputParser.build_set(0x18, 0x1a, 0x80..0x8f, 0x91..0x97, 0x99, 0x9a, 0x9c)
        m.define_transition from: nil, to: OscString, code_set: TtyOutputParser.build_set(0x9d)
        m.define_transition from: nil, to: SosPmApcString, code_set: TtyOutputParser.build_set(0x98, 0x9e, 0x9f)

        @state_machine = m
      end

      @state_machine
    end

    class State
      attr_accessor :parser

      def initialize(parser)
        self.parser = parser
      end

      def enter
        TtyOutputParser.logger.debug "#{self.class} enter"

        if action = self.class.enter_action
          parser.send action
        end
      end

      class_attribute(*TtyOutputParser::ACTIONS)
      class_attribute :enter_action, :exit_action

      def event(byte)
        TtyOutputParser.logger.debug "#{self.class} event #{StateMachine.code_to_hex_s byte} (#{TtyOutputParser.byte_to_s(byte).inspect})"

        TtyOutputParser::ACTIONS.each do |action|
          if self.class.send(action) && self.class.send(action).include?(byte)
            parser.send action, byte
          end
        end
      end

      def exit
        TtyOutputParser.logger.debug "#{self.class} exit"
        if action = self.class.exit_action
          parser.send action
        end
      end
    end

    class Ground < State
      self.print = TtyOutputParser.build_set 0x20..0x7f, 0xa0..0xff
      self.execute = TtyOutputParser.build_set 0x00..0x17, 0x19, 0x1c..0x1f
    end
    class CsiEntry < State
      self.enter_action = :clear
      self.execute = TtyOutputParser.build_set 0x00..0x17, 0x19, 0x1c..0x1f
      self.ignore = TtyOutputParser.build_set 0x7f
    end
    class CsiIgnore < State
      self.execute = TtyOutputParser.build_set 0x00..0x17, 0x19, 0x1c..0x1f
      self.ignore = TtyOutputParser.build_set 0x20..0x3F, 0x7f
    end
    class CsiIntermediate < State
      self.collect = TtyOutputParser.build_set 0x20..0x2f
      self.execute = TtyOutputParser.build_set 0x00..0x17, 0x19, 0x1c..0x1f
      self.ignore = TtyOutputParser.build_set 0x7f
    end
    class CsiParam < State
      self.execute = TtyOutputParser.build_set 0x00..0x17, 0x19, 0x1c..0x1f
      self.param = TtyOutputParser.build_set 0x30..0x39, 0x3b
      self.ignore = TtyOutputParser.build_set 0x7f
    end
    class DcsEntry < State
      self.enter_action = :clear
      self.ignore = TtyOutputParser.build_set 0x00..0x17, 0x19, 0x1c..0x1f, 0x7f
    end
    class DcsIgnore < State
      self.ignore = TtyOutputParser.build_set 0x00..0x17, 0x19, 0x1c..0x1f, 0x20..0x7f
    end
    class DcsIntermediate < State
      self.ignore = TtyOutputParser.build_set 0x00..0x17, 0x19, 0x1c..0x1f, 0x7f
      self.collect = TtyOutputParser.build_set 0x20..0x2f
    end
    class DcsParam < State
      self.ignore = TtyOutputParser.build_set 0x00..0x17, 0x19, 0x1c..0x1f, 0x7f
      self.param = TtyOutputParser.build_set 0x30..0x39, 0x3b
    end
    class DcsPassthrough < State
      self.enter_action = :hook
      self.put = TtyOutputParser.build_set 0x00..0x17, 0x19, 0x1c..0x1f, 0x20..0x7e
      self.ignore = TtyOutputParser.build_set 0x7f
      self.exit_action = :unhook
    end
    class Escape < State
      self.enter_action = :clear
      self.execute = TtyOutputParser.build_set 0x00..0x17, 0x19, 0x1c..0x1f
      self.ignore = TtyOutputParser.build_set 0x7f
    end
    class EscapeIntermediate < State
      self.execute = TtyOutputParser.build_set 0x00..0x17, 0x19, 0x1c..0x1f
      self.collect = TtyOutputParser.build_set 0x20..0x2f
      self.ignore = TtyOutputParser.build_set 0x7f
    end
    class OscString < State
      self.enter_action = :osc_start
      self.ignore = TtyOutputParser.build_set 0x00..0x17, 0x19, 0x1c..0x1f
      self.osc_put = TtyOutputParser.build_set 0x20..0x7f
      self.exit_action = :osc_end
    end
    class SosPmApcString < State
      self.ignore = TtyOutputParser.build_set 0x00..0x17, 0x19, 0x1c..0x1f, 0x20..0x7f
    end

    # Just one class for transitions since we don't care about named subclasses
    class Transition
      attr_accessor :parser, :from, :to, :code_set, :action

      def initialize(parser:, from:nil, to:, code_set:, action:nil)
        self.code_set = code_set
        self.from     = from
        self.parser   = parser
        self.to       = to
        self.action   = action
      end

      def event(byte)
        TtyOutputParser.logger.debug "#{self} event: #{StateMachine.code_to_hex_s byte} (#{TtyOutputParser.byte_to_s(byte).inspect})"
        parser.send action, byte if action
        true
      end

      def can?(current_state, byte)
        (!from || from == current_state) && code_set.include?(byte)
      end

      def to_s
        "#<Transition from: #{from.try :class} to: #{to.class} action:#{action}>"
      end
    end

    class StateMachine
      attr_accessor :parser, :states, :current_state, :transitions

      def initialize(parser, state_classes, initial_state_class)
        self.parser = parser
        self.states = state_classes.map {|s| s.new parser }
        self.current_state = states.find {|s| initial_state_class === s }
        self.transitions = []
      end

      # Create transaction and add it to state machine.
      def define_transition(from:nil, to:, code_set:, action: nil, &block)
        transitions << Transition.new(
          action: action,
          code_set: code_set,
          from: states.find {|s| from === s },
          parser: parser,
          to: states.find {|s| to === s },
        )
      end

      # Run the state machine by feeding in byte values, one by one
      def byte=(byte)
        if transition = transitions.find {|t| t.can? current_state, byte }
          current_state.exit
          transition.event byte
          self.current_state = states.find {|s| transition.to === s }
          current_state.enter

        else
          current_state.event byte
        end

        byte
      end

      def self.code_to_hex_s(code)
        "0x%02X" % code
      end
    end
  end
end
