require 'active_support/all'

module Intermix
  module TtyCodes
    # abstract class
    class Sequence
      attr_accessor \
        :chars,
        :codes,
        :long_name,
        :capname

      def initialize(options={})
        options.each {|m, v|
          send "#{m}=", v
        }
      end

      def codes=(codes)
        @chars = codes.map{|c| code_to_char c }
        @codes = codes
      end

      def chars=(chars)
        @codes = chars.map{|c| char_to_code c }
        @chars = chars
      end

      private

      def code_to_char(code)
        [code].pack 'U'
      end

      def char_to_code(char, raise_on_multibyte:true)
        bytes = char.to_s.each_byte.to_a
        bytes.length == 1 or raise "char must be one byte" if raise_on_multibyte
        bytes.first
      end
    end

    class Escape < Sequence; end
    class Control < Sequence; end

    class Csi < Sequence
      # like: %(1 5) for "\e[1;5h"
      attr_accessor :param_chars

      # asci values for the params
      attr_accessor :param_codes

      def param_codes=(param_codes)
        param_codes or return
        @param_chars = param_codes.map{|c| code_to_char c }
        @param_codes = param_codes
      end

      def param_chars=(param_chars)
        param_chars or return
        @param_codes = param_chars.map{|c| char_to_code c, raise_on_multibyte: false }
        @param_chars = param_chars
      end
    end

    def self.csi(chars, param_chars, capname, long_name)
      (@csis ||= []) << Csi.new(capname: capname, long_name: long_name, chars: chars, param_chars: param_chars)
    end

    def self.escape(capname, long_name, chars)
      (@escapes ||= []) << Escape.new(capname: capname, long_name: long_name, chars: chars)
    end

    def self.control(capname, long_name, chars)
      (@controls ||= []) << Control.new(capname: capname, long_name: long_name, chars: chars)
    end

    def self.find_csi(codes:, param_chars:)
      @csis.find {|s| s.codes == codes && s.param_chars == param_chars }
    end

    def self.find_escape(codes:)
      @escapes.find {|s| s.codes == codes }
    end
    def self.find_control(codes:)
      @controls.find {|s| s.codes == codes }
    end

    # Harvested from infocmp then hand edited
    #csi "setab",      'set_a_background',       %w(m), %w(%?%p1%{8}%<%t4%p1%d%e%p1%{16}%<%t10%p1%{8}%-%d%e48;5;%p1%d%;m"
    #csi 'flash',      'flash_screen',            %w(l), "\e[?5h$<100/>\e[?5l"
    #csi 'is2',        nil,                       %w(>), "\e[!p\e[?3;4l\e[4l\e>"
    #csi 'rmkx',       'keypad_local',           %w(l), %w(?1l\e>"
    #csi 'rs2',        nil,                      %w(l), %w(!p\e[?3;4l\e[4l\e>"
    #csi 'setaf',      'set_a_foreground',       %w(m), %w(%?%p1%{8}%<%t3%p1%d%e%p1%{16}%<%t9%p1%{8}%-%d%e38;5;%p1%d%;m"
    #csi 'sgr',        'set_attributes',         %w(m), %w(p9%t\e(0%e\e(B%;\e[0%?%p6%t;1%;%?%p2%t;4%;%?%p1%p3%|%t;7%;%?%p4%t;5%;%?%p7%t;8%;m"
    #csi 'sgr0',       'exit_attribute_mode',    %w(m), %w(B\e[m"
    #csi 'smkx',       'keypad_xmit',            %w(h), %w(?1h\e="
    #escape 'kb2',     'key_b2',                  %w(x), "\eOE"
    #escape 'kcub1',   'key_left',                %w(x), "\eOD"
    #escape 'kcud1',   'key_down',                %w(x), "\eOB"
    #escape 'kcuf1',   'key_right',               %w(x), "\eOC"
    #escape 'kcuu1',   'key_up',                  %w(x), "\eOA"
    #escape 'kend',    'key_end',                 %w(x), "\eOF"
    #escape 'kent',    'key_enter',               %w(x), "\eOM"
    #escape 'kf1',     'key_f1',                  %w(x), "\eOP"
    #escape 'kf2',     'key_f2',                  %w(x), "\eOQ"
    #escape 'kf3',     'key_f3',                  %w(x), "\eOR"
    #escape 'kf4',     'key_f4',                  %w(x), "\eOS"
    #escape 'khome',   'key_home',                %w(x), "\eOH"
    #osc 'initc',      nil,                       "\e]4;%p1%d;rgb\:%p2%{255}%*%{1000}%/%2.2X/%p3%{255}%*%{1000}%/%2.2X/%p4%{255}%*%{1000}%/%2.2X\e\\"
    #unknown "rmacs",  'exit_alt_charset_mode',   "\e(B"
    #unknown 'smacs',  'enter_alt_charset_mode',  "\e(0"
    control 'bel',   'bell',             %w(G)
    control 'cr',    'carriage_return',  %w(M)
    control 'cub1',  'key_backspace',    %w(H)
    control 'cud1',  'scroll_forward',   %w(J)
    control 'ht',    'tab',              %w(I)
    control 'ind',   nil,                %w(J)
    control 'kbs',   nil,                %w(H)
    control nil,     'backspace',        ["\b"]
    control 'cr',    'carriage_return',  ["\r"]
    control nil,     'formfeed',         ["\f"]
    control nil,     'linefeed',         ["\l"]
    control 'nl',    'newline',          ["\n"]
    control nil,     'space',            ["\s"]
    control nil,     'tab',              ["\t"]

    csi %w(? c),  %w(1 2),    'u8',     'user8'
    csi %w(? h),  %w(1034),   'smm',    'meta_on'
    csi %w(? h),  %w(1049),   'smcup',  'enter_ca_mode'
    csi %w(? h),  %w(7),      'smam',   'enter_am_mode'
    csi %w(? l),  %w(1034),   'rmm',    'meta_off'
    csi %w(? l),  %w(1049),   'rmcup',  'exit_ca_mode'
    csi %w(? l),  %w(7),      'rmam',   'exit_am_mode'
    csi %w(@),    [],        'ich',    'parm_ich'
    csi %w(A),    %w(1 2),    'kri',    'key_sr'
    csi %w(A),    [],        'cuu1',   'cursor_up'
    csi %w(B),    %w(1 2),    'kind',   'key_sf'
    csi %w(B),    [],        'cud',    'parm_down_cursor'
    csi %w(C),    %w(1 2),    'kRIT',   'key_sright'
    csi %w(C),    [],        'cuf',    'parm_right_cursor'
    csi %w(D),    %w(1 2),    'kLFT',   'key_sleft'
    csi %w(D),    [],        'cub',    'parm_left_cursor'
    csi %w(F),    %w(1 2),    'kEND',   'key_send'
    csi %w(G),    [],        'hpa',    'column_address'
    csi %w(H),    %w(1 2),    'kHOM',   'key_shome'
    csi %w(H),    [],        'cup',    'cursor_address'
    csi %w(H),    [],        'home',   'cursor_home'
    csi %w(J),    [],        'ed',     'clr_eos'
    csi %w(K),    [],        'elr',    'clr_eol_right'
    csi %w(K),    %w(0),      'elr',    'clr_eol_right'
    csi %w(K),    %w(1),      'ell',    'clr_eol_left'
    csi %w(K),    %w(2),      'ela',    'clr_eol_all'
    csi %w(L),    [],        'il',     'parm_insert_line'
    csi %w(L),    [],        'il1',    'insert_line'
    csi %w(M),    [],        'dl',     'parm_delete_line'
    csi %w(M),    [],        'dl1',    'key_mouse'
    csi %w(M),    [],        'kmous',  nil
    csi %w(P),    %w(1 2),    'kf13',   'key_f13'
    csi %w(P),    %w(1 3),    'kf49',   'key_f49'
    csi %w(P),    %w(1 4),    'kf61',   'key_f61'
    csi %w(P),    %w(1 5),    'kf25',   'key_f25'
    csi %w(P),    %w(1 6),    'kf37',   'key_f37'
    csi %w(P),    [],        'dch',    'parm_dch'
    csi %w(P),    [],        'dch1',   'delete_character'
    csi %w(Q),    %w(1 2),    'kf14',   'key_f14'
    csi %w(Q),    %w(1 3),    'kf50',   'key_f50'
    csi %w(Q),    %w(1 4),    'kf62',   'key_f62'
    csi %w(Q),    %w(1 5),    'kf26',   'key_f26'
    csi %w(Q),    %w(1 6),    'kf38',   'key_f38'
    csi %w(R),    %w(1 2),    'kf15',   'key_f15'
    csi %w(R),    %w(1 3),    'kf51',   'key_f51'
    csi %w(R),    %w(1 4),    'kf63',   'key_f63'
    csi %w(R),    %w(1 5),    'kf27',   'key_f27'
    csi %w(R),    %w(1 6),    'kf39',   'key_f39'
    csi %w(R),    [],        'u6',     'user6'
    csi %w(S),    %w(1 2),    'kf16',   'key_f16'
    csi %w(S),    %w(1 3),    'kf52',   'key_f52'
    csi %w(S),    %w(1 5),    'kf28',   'key_f28'
    csi %w(S),    %w(1 6),    'kf40',   'key_f40'
    csi %w(S),    [],        'indn',   'parm_index'
    csi %w(T),    [],        'rin',    'parm_rindex'
    csi %w(X),    [],        'ech',    'erase_chars'
    csi %w(Z),    [],        'cbt',    'key_btab'
    csi %w(Z),    [],        'kcbt',   nil
    csi %w(c),    [],        'u9',     'user9'
    csi %w(d),    [],        'vpa',    'row_address'
    csi %w(g),    %w(3),      'tbc',    'clear_all_tabs'
    csi %w(h),    %w(1),      nil,      nil
    csi %w(h),    %w(12 25),  'cvvis',  'cursor_visible'
    csi %w(h),    %w(4),      'smir',   'enter_insert_mode'
    csi %w(i),    %w(4),      'mc4',    'prtr_off'
    csi %w(i),    %w(5),      'mc5',    'prtr_on'
    csi %w(i),    [],        'mc0',    'print_screen'
    csi %w(l),    %w(25),     'civis',  'cursor_invisible'
    csi %w(l),    %w(4),      'rmir',   'exit_insert_mode'
    csi %w(m),    %w(1),      'bold',   'enter_bold_mode'
    csi %w(m),    %w(24),     'rmul',   'exit_underline_mode'
    csi %w(m),    %w(27),     'rmso',   'exit_standout_mode'
    csi %w(m),    %w(39 49),  'op',     'orig_pair'
    csi %w(m),    %w(4),      "smul",   'enter_underline_mode'
    csi %w(m),    %w(5),      'blink',  'enter_blink_mode'
    csi %w(m),    %w(7),      'rev',    'enter_standout_mode'
    csi %w(m),    %w(7),      'smso',   nil
    csi %w(m),    %w(8),      'invis',  'enter_secure_mode'
    csi %w(n),    %w(6),      'u7',     'user7'
    csi %w(r),    [],        'csr',    'change_scroll_region'
    csi %w(~),    %w(15 2),   'kf17',   'key_f17'
    csi %w(~),    %w(15 3),   'kf53',   'key_f53'
    csi %w(~),    %w(15 5),   'kf29',   'key_f29'
    csi %w(~),    %w(15 6),   'kf41',   'key_f41'
    csi %w(~),    %w(15),     'kf5',    'key_f5'
    csi %w(~),    %w(17 2),   'kf18',   'key_f18'
    csi %w(~),    %w(17 3),   'kf54',   'key_f54'
    csi %w(~),    %w(17 5),   'kf30',   'key_f30'
    csi %w(~),    %w(17 6),   'kf42',   'key_f42'
    csi %w(~),    %w(17),     'kf6',    'key_f6'
    csi %w(~),    %w(18 2),   'kf19',   'key_f19'
    csi %w(~),    %w(18 3),   'kf55',   'key_f55'
    csi %w(~),    %w(18 5),   'kf31',   'key_f31'
    csi %w(~),    %w(18 6),   'kf43',   'key_f43'
    csi %w(~),    %w(18),     'kf7',    'key_f7'
    csi %w(~),    %w(19 2),   'kf20',   'key_f20'
    csi %w(~),    %w(19 3),   'kf56',   'key_f56'
    csi %w(~),    %w(19 5),   'kf32',   'key_f32'
    csi %w(~),    %w(19 6),   'kf44',   'key_f44'
    csi %w(~),    %w(19),     'kf8',    'key_f8'
    csi %w(~),    %w(2 2),    'kIC',    'key_sic'
    csi %w(~),    %w(2),      'kich1',  'key_ic'
    csi %w(~),    %w(20 2),   'kf21',   'key_f21'
    csi %w(~),    %w(20 3),   'kf57',   'key_f57'
    csi %w(~),    %w(20 5),   'kf33',   'key_f33'
    csi %w(~),    %w(20 6),   'kf45',   'key_f45'
    csi %w(~),    %w(20),     'kf9',    'key_f9'
    csi %w(~),    %w(21 2),   'kf22',   'key_f22'
    csi %w(~),    %w(21 3),   'kf58',   'key_f58'
    csi %w(~),    %w(21 5),   'kf34',   'key_f34'
    csi %w(~),    %w(21 6),   'kf46',   'key_f46'
    csi %w(~),    %w(21),     'kf10',   'key_f10'
    csi %w(~),    %w(23 2),   'kf23',   'key_f23'
    csi %w(~),    %w(23 3),   'kf59',   'key_f59'
    csi %w(~),    %w(23 5),   'kf35',   'key_f35'
    csi %w(~),    %w(23 6),   'kf47',   'key_f47'
    csi %w(~),    %w(23),     'kf11',   'key_f11'
    csi %w(~),    %w(24 2),   'kf24',   'key_f24'
    csi %w(~),    %w(24 3),   'kf60',   'key_f60'
    csi %w(~),    %w(24 5),   'kf36',   'key_f36'
    csi %w(~),    %w(24 6),   'kf48',   'key_f48'
    csi %w(~),    %w(24),     'kf12',   'key_f12'
    csi %w(~),    %w(3 2),    "kDC",    'key_sdc'
    csi %w(~),    %w(3),      "kdch1",  'key_dc'
    csi %w(~),    %w(5 2),    'kPRV',   'key_sprevious'
    csi %w(~),    %w(5),      'kpp',    'key_ppage'
    csi %w(~),    %w(6 2),    'kNXT',   'key_snext'
    csi %w(~),    %w(6),      "knp",    'key_npage'

    escape 'hts',   'set_tab',         %w(H)
    escape 'meml',  'memory_lock',     %w(l)
    escape 'memu',  'memory_unlock',   %w(m)
    escape 'rc',    'restore_cursor',  %w(8)
    escape 'ri',    'scroll_reverse',  %w(M)
    escape 'rs1',   'reset_1string',   %w(c)
    escape 'sc',    'save_cursor',     %w(7)
  end
end
