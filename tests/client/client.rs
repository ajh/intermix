use ::support::test_io::*;
use libintermix::client::*;
use regex::Regex;
use std::sync::{Arc, Mutex};
use time;
use vterm_sys::*;
use std::io::prelude::*;

// Build a vterm instance
fn build_vterm(size: ScreenSize) -> VTerm {
    let mut vterm = VTerm::new(size);
    vterm.state.set_default_colors(Color { red: 230, green: 230, blue: 230 },
                                   Color { red: 5, green: 5, blue: 5 });
    vterm.state.reset(true);
    vterm
}

fn assert_in_command_mode<T: Read>(vterm: &mut VTerm, reader: &mut T) {
    ::try_until_true(|| {
        let mut bytes: Vec<u8> = vec![];
        reader.read_to_end(&mut bytes);
        vterm.write(&bytes);

        let actual = vterm.screen.get_text(Rect { start_row: 25, end_row: 26, start_col: 0, end_col: 80 });
        let re = Regex::new(r"command-mode").unwrap();
        re.is_match(&actual)
    });
}

#[test]
fn client_can_enter_command_mode() {
    let mut io = TestIO::new();
    let (client_tx, client) = Client::spawn(::std::io::stdin(), io.clone());

    // The screen size here is hard coded through the client code. Need to fix that.
    let mut vterm = build_vterm(ScreenSize { rows: 26, cols: 80 });

    assert_in_command_mode(&mut vterm, &mut io);

    client.stop();
}
