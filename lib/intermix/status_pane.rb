module Intermix
  class StatusPane < Pane
    def initialize(window)
      super 1,  window.cols, window.rows - 1, 0, "status"
      self.screen = Screen.new size_rows, size_cols, TermInfo.new(ENV['TERM'], STDOUT)
    end

    def status=(val)
      @screen.pen.move_left full: true
      val.each_byte {|byte| @screen.print byte}
    end
  end
end
