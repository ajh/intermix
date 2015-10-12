module Intermix

  # a window pane, which is a rectangular section of the screen that the user
  # sees
  class Pane
    attr_accessor :program, :size_rows, :size_cols, :offset_row, :offset_col, :name, :transforms

    def initialize(size_rows, size_cols, offset_row, offset_col, name="")
      self.size_rows = size_rows
      self.size_cols = size_cols
      self.offset_row = offset_row
      self.offset_col = offset_col
      self.name = name
      self.transforms = []
    end

    def screen=(screen)
      @screen = screen
    end

    def screen
      transforms.any? ? transforms.first : @screen
    end

    def inspect
      "#<#{self.class} name=#{name} size_rows=#{size_rows} size_cols=#{size_cols} offset_row=#{offset_row} offset_col=#{offset_col} screen=#{@screen} transforms=#{transforms}>"
    end
  end
end
