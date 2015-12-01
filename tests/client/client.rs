use regex::Regex;
use super::support::capture_io::*;
use libintermix::client::*;
use vterm_sys::*;

// Build a vterm instance
fn build_vterm(size: ScreenSize) -> VTerm {
    let mut vterm = VTerm::new(size);
    vterm.state.set_default_colors(Color { red: 230, green: 230, blue: 230 },
                                   Color { red: 5, green: 5, blue: 5 });
    vterm.state.reset(true);
    vterm
}

#[test]
fn client_starts_in_command_mode() {
    ::setup_logging();

    // create capture io
    let (mut io, bytes) = CaptureIO::new();

    // create & start client
    let (client_tx, client_handle) = Client::spawn(io);

    // quit client
    client_tx.send(ClientMsg::Quit).unwrap();

    // need to improve quit behavior first
    //client_handle.join().unwrap();
    ::std::thread::sleep_ms(3000);

    // The screen size here is hard coded through the client code. Need to fix that.
    let mut vterm = build_vterm(ScreenSize { rows: 26, cols: 80 });
    vterm.write(&bytes.lock().unwrap());

    let actual = vterm.screen.get_text(Rect { start_row: 25, end_row: 26, start_col: 0, end_col: 80 });
    println!("{}", actual);
    // expect command-mode to be displayed
    let re = Regex::new(r"command-mode").unwrap();
    assert!(re.is_match(&actual));
}
