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

        let actual = vterm.screen.get_text(Rect { start_row: 0, end_row: 24, start_col: 0, end_col: 80 });
        let re = Regex::new(r"command-mode").unwrap();
        re.is_match(&actual)
    });
}

fn assert_in_program_mode<T: Read>(vterm: &mut VTerm, reader: &mut T) {
    ::try_until_true(|| {
        let mut bytes: Vec<u8> = vec![];
        reader.read_to_end(&mut bytes);
        vterm.write(&bytes);

        let actual = vterm.screen.get_text(Rect { start_row: 23, end_row: 24, start_col: 0, end_col: 80 });
        let re = Regex::new(r"program-mode").unwrap();
        re.is_match(&actual)
    });
}

#[test]
fn client_starts_in_command_mode() {
    ::setup_logging();
    let mut output = TestIO::new();
    let mut input = TestIO::new();

    let (client_tx, client) = Client::spawn(input.clone(), output.clone(), TtyIoCtlConfig { rows: 24, cols: 80 });

    // The screen size here is hard coded through the client code. Need to fix that.
    let mut vterm = build_vterm(ScreenSize { rows: 24, cols: 80 });

    assert_in_command_mode(&mut vterm, &mut output);

    client.stop();
}

#[test]
fn client_can_enter_program_mode() {
    ::setup_logging();
    let mut output = TestIO::new();
    let mut input = TestIO::new();

    let (client_tx, client) = Client::spawn(input.clone(), output.clone(), TtyIoCtlConfig { rows: 24, cols: 80 });

    // The screen size here is hard coded through the client code. Need to fix that.
    let mut vterm = build_vterm(ScreenSize { rows: 24, cols: 80 });

    assert_in_command_mode(&mut vterm, &mut output);
    input.write(b"r");
    // sending this message is hacky
    client_tx.send(ClientMsg::ProgramAdd {
        server_id: "some server".to_string(),
        program: state::Program { id: "123".to_string(), is_subscribed: true },
    });
    assert_in_program_mode(&mut vterm, &mut output);

    client.stop();
}
