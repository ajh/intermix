require 'pp'

module Intermix
  class Screen
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

      def dirty?
        dirty == true
      end
    end
  end
end
