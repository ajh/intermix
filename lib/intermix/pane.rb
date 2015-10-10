module Intermix

  # a window pane, which is a rectangular section of the screen that the user
  # sees
  class Pane
    attr_accessor :program, :size_rows, :size_cols, :offset_row, :offset_col

    def initialize(size_rows, size_cols, offset_row, offset_col)
      self.size_rows = size_rows
      self.size_cols = size_cols
      self.offset_row = offset_row
      self.offset_col = offset_col
    end
  end
end
