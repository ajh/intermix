module Intermix
  # A class that displays intermix to the user's screen.
  class Window
    attr_accessor :io, :terminfo

    def initialize(io)
      self.io = io
      self.terminfo = TermInfo.new ENV['TERM'], io
    end

    def start
      io.sync = true
      terminfo.control 'smcup'
    end

    def close
      terminfo.control 'rmcup'
    end

    def rows
      terminfo.screen_lines
    end

    def cols
      terminfo.screen_columns
    end

    # Painters may paint in sections of the window by using the offsets
    def paint(screen, offset_row=0, offset_col=0)
      last_row = last_col = 0

      screen.cells.each do |cell|
        cell.dirty or next
        cell.code or next

        current_row = cell.row + offset_row
        current_col = cell.col + offset_col
        already_in_position = current_row == last_row || current_col == last_col + 1

        if !already_in_position
          terminfo.control 'cup', cell.row + offset_row, cell.col + offset_col
        end

        last_row = current_row
        last_col = current_col

        #if cell.bold
        #terminfo.control 'bold'
        #end

        io.print "#{cell.char}"

        # Inefficient to clear the attribute each time
        #terminfo.control 'sgr0'
      end

      screen.clean_all
    end
  end
end
