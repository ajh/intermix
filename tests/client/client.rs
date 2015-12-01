use libintermix::client::*;
use regex::Regex;
use std::sync::{Arc, Mutex};
use super::support::capture_io::*;
use time;
use vterm_sys::*;

// Build a vterm instance
fn build_vterm(size: ScreenSize) -> VTerm {
    let mut vterm = VTerm::new(size);
    vterm.state.set_default_colors(Color { red: 230, green: 230, blue: 230 },
                                   Color { red: 5, green: 5, blue: 5 });
    vterm.state.reset(true);
    vterm
}

fn assert_in_command_mode(mut vterm: VTerm, tty_output: Arc<Mutex<Vec<u8>>>) {
    ::try_until_true(|| {
        vterm.write(&tty_output.lock().unwrap());
        let actual = vterm.screen.get_text(Rect { start_row: 25, end_row: 26, start_col: 0, end_col: 80 });
        let re = Regex::new(r"command-mode").unwrap();
        re.is_match(&actual)
    });
}

#[test]
fn client_can_enter_command_mode() {
    let (mut io, tty_output) = CaptureIO::new();
    let (client_tx, client) = Client::spawn(io);

    // The screen size here is hard coded through the client code. Need to fix that.
    let mut vterm = build_vterm(ScreenSize { rows: 26, cols: 80 });

    assert_in_command_mode(vterm, tty_output);

    client.stop();
}
