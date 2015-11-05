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
        tty.apply_cap("smcup", &[]).unwrap();
    }

    /// this isn't working for some reason
    pub fn stop(&self) {
        let mut tty = TerminfoTerminal::new(io::stdout()).unwrap();
        tty.apply_cap("rmcup", &[]).unwrap();
        terminfo::set_cooked_mode(0);
    }
}
