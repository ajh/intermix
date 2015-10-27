use std::io;
use ::terminfo;
use term::terminfo::*;

pub struct Window {
    something: bool,
}

impl Window {
    pub fn new() -> Window {
        Window { something: true }
    }

    pub fn start(&self) {
        terminfo::set_raw_mode(0);
        let mut tty = TerminfoTerminal::new(io::stdout()).unwrap();
        tty.apply_cap("smcup", &[]);
    }

    pub fn stop(&self) {
        terminfo::set_cooked_mode(0);
        let mut tty = TerminfoTerminal::new(io::stdout()).unwrap();
        tty.apply_cap("rmcup", &[]);
    }

    pub fn rows_count(&self) -> usize {
      let (rows_count, _) = terminfo::get_win_size(0).unwrap();
      rows_count as usize
    }

    pub fn cols_count(&self) -> usize {
      let (_, cols_count) = terminfo::get_win_size(0).unwrap();
      cols_count as usize
    }
}
