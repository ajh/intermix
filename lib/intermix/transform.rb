require 'delegate'

module Intermix
  # A base class for transforms. It has a #cells
  # method like a screen, but potentialy
  # transforms the data.
  class Transform < SimpleDelegator
    def initialize(cell_iterator)
      __setobj__ cell_iterator
    end

    def inspect
      "#<#{self.class} __getobj__=#{__getobj__}>"
    end
  end

  # Doesn't do anything to the cells.
  class NoTransform < Transform
  end

  class CropTransform < Transform
    attr_accessor :size_rows, :size_cols, :offset_rows, :offset_cols

    def initialize(cell_iterator, size_rows, size_cols, offset_rows, offset_cols)
      super cell_iterator
      self.size_rows = size_rows
      self.size_cols = size_cols
      self.offset_rows = offset_rows
      self.offset_cols = offset_cols
    end

    def cells
      iterator = super
      Enumerator.new do |yielder|
        loop do
          begin
            cell = iterator.next
          rescue StopIteration
            break
          end

          row_visible = (offset_rows..offset_rows+size_rows).include? cell.row
          col_visible = (offset_cols..offset_cols+size_cols).include? cell.col
          if row_visible && col_visible
            yielder << cell
          end
        end
      end
    end
  end
end
