require 'pp'

module Intermix
  # this class stores the state of a video terminal. This can be used in combo
  # with a control sequence parser to maintain the state of a psuedo terminal.
  # The state is useful for displaying or transforming.
  class Screen
    attr_accessor :buffer, :pen, :terminfo, :size_cols, :size_rows

    def self.logger
      @logger ||= Logger.new('log/screen.log').tap do |l|
        #l.sev_threshold = Logger::WARN
      end
    end

    def initialize(size_rows, size_cols, terminfo)
      self.size_cols = size_cols
      self.size_rows = size_rows
      self.terminfo = terminfo

      # buffer is a 2D array where rows are first and columns are nested inside
      # rows.
      self.buffer = []
      size_rows.times do |r|
        buffer << (row = [])
        size_cols.times do |c|
          row << Cell.new(row: r, col: c)
        end
      end

      self.pen = Pen.new 0, 0, size_rows, size_cols
    end

    def print(code)
      buffer[pen.row][pen.col].draw code, pen
      pen.move_right
    end

    def ht
      tabwidth = terminfo.tigetnum 'it'
      spaces_needed = tabwidth - pen.col % tabwidth
      spaces_needed.times { print 0x20 }
    end

    def backspace
      pen.move_left
    end

    def erase_in_line(direction: :right)
      if direction == :right
        buffer[pen.row][pen.col].draw 0x20, pen
      end
    end

    def save_cursor
    end

    def alternate_screen_buffer
    end

    def normal_screen_buffer
    end

    # Returns a Enumerator of cells in row order
    def cells
      Enumerator.new do |yielder|
        buffer.each do |row|
          row.each do |cell|
            yielder << cell
          end
        end
      end
    end

    def inspect
      string = "#<#{self.class} buffer: [\n"
      buffer.each do |row|
        row.each do |cell|
          string += cell.char
        end
        string += "\n"
      end
      string + "\n] >"
    end

    private

    class Cell
      attr_accessor \
        :code,
        :col,
        :dirty,
        :row

      CHARACTER_ATTRS = [
        :blink,
        :bold,
        :faint,
      ].freeze
      attr_accessor(*CHARACTER_ATTRS)

      def initialize(code:nil, row:, col:)
        self.row = row
        self.col = col
        self.code = code
        self.dirty = false
      end

      # return the code as a one character ruby string
      def char
        code ? [code].pack('U') : ' '
      end

      def draw(code, pen)
        self.code = code
        self.dirty = true
        CHARACTER_ATTRS.each do |attr|
          send "#{attr}=", pen.send(attr) if pen.respond_to?(attr)
        end
      end
    end

    class Pen
      attr_accessor \
        :blink,
        :bold,
        :col,
        :faint,
        :inverse,
        :italicized,
        :max_col,
        :max_row,
        :row,
        :underlined
      # ... many more

      def self.logger; @logger ||= Screen.logger; end

      def initialize(row, col, max_row, max_col)
        self.col = col
        self.row = row
        self.max_row = max_row
        self.max_col = max_col
      end

      def move_right(count:1, full: false)
        if full
          self.col = max_col - 1
        else
          self.col += count

          if col > (max_col - 1)
            self.class.logger.error "#{__method__} bumped edge of buffer"
            self.col = max_col - 1
          end
        end
        self.class.logger.debug "#{__method__} #{count} full:#{full}. col now #{col}"
      end

      def move_left(count:1, full: false)
        if full
          self.col = 0
        else
          self.col -= count
          if col < 0
            self.class.logger.error "#{__method__} bumped edge of buffer"
            self.col = 0
          end
        end
        self.class.logger.debug "#{__method__} #{count} full:#{full}. col now #{col}"
      end

      def move_up(count:1, full: false)
        if full
          self.row = 0
        else
          self.row -= count
          if col < 0
            self.class.logger.error "#{__method__} bumped edge of buffer"
            self.col = 0
          end
        end
        self.class.logger.debug "#{__method__} #{count} full:#{full}. row now #{row}"
      end

      def move_down(count:1, full: false)
        if full
          self.row = max_row - 1
        else
          self.row += count
          if row > (max_row - 1)
            self.class.logger.error "#{__method__} bumped edge of buffer"
            self.row = max_row - 1
          end
        end
        self.class.logger.debug "#{__method__} #{count} full:#{full}. row now #{row}"
      end
    end
  end
end
