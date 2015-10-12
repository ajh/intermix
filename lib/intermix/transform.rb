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

    def self.logger
      @logger ||= Logger.new('log/transform.log').tap do |l|
        #l.sev_threshold = Logger::WARN
      end
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

          if visible_rows.include?(cell.row) && visible_cols.include?(cell.col)
            yielder << cell
          end
        end
      end
    end

    private

    def visible_rows
      offset_rows..offset_rows+size_rows
    end

    def visible_cols
      offset_cols..offset_cols+size_cols
    end
  end

  class SampleTransform < Transform
    attr_accessor :size_rows, :size_cols

    def initialize(cell_iterator, size_rows, size_cols)
      super cell_iterator
      self.size_rows = size_rows
      self.size_cols = size_cols

      @cells = build_cells
    end

    def cells
      Enumerator.new do |yielder|
        @cells.each_with_index do |row, row_num|
          row.each_with_index do |cell, col_num|
            sampled_cells = cells_in_sample row_num, col_num
            if sampled_cells.any?(&:dirty?)
              sampled_cells.each {|c| c.dirty = false }
              cell.code = sample row_num, col_num
              cell.dirty = true
            end

            yielder << cell
          end
        end
      end
    end

    private

    # Samples a rectangle of cells in buffer and returns a code
    def sample(row, col)
      cells = cells_in_sample row, col

      visible_cells = cells.select {|c| char_is_visible? c.char }
      mostly_visible = visible_cells.length >= cells.length / 4

      if mostly_visible
        freq = visible_cells.inject(Hash.new(0)) {|h, v| h[v.code] += 1; h}
        visible_cells.max_by { |v| freq[v.code] }.code
      else
        0x20 # space
      end
    end

    # Return an array of cells in the sample identified by the row and column
    # index
    def cells_in_sample(row, col)
      width_of_rect = __getobj__.size_rows / size_rows
      height_of_rect = __getobj__.size_cols / size_cols

      cells_in_rect (row * width_of_rect) - (width_of_rect / 2),
                    (col * height_of_rect) - (height_of_rect / 2),
                    width_of_rect,
                    height_of_rect
    end

    def cells_in_rect(row, col, width, height)
      x = row - (width / 2)
      x = 0 if x < 0
      y = col - (height / 2)
      y = 0 if y < 0

      buffer[x..x+width].map {|slice| slice[y..y+height]}.flatten
    end

    def char_is_visible?(char)
      ('a'..'z').include?(char) ||
        ('A'..'Z').include?(char) ||
        ('0'..'9').include?(char)
    end

    def build_cells
      size_rows.times.map do |row|
        size_cols.times.map do |col|
          Screen::Cell.new code: nil, row: row, col: col
        end
      end
    end
  end
end
