use ::support::*;
use libintermix::client::*;
use regex::Regex;
use vterm_sys::*;
use std::io::prelude::*;
use std::sync::mpsc::Sender;

const CTRL_B: u8 = 2u8;

// Build a vterm instance
fn build_vterm(size: &ScreenSize) -> VTerm {
    let mut vterm = VTerm::new(size);
    vterm.state_set_default_colors(&ColorRGB {
                                       red: 230,
                                       green: 230,
                                       blue: 230,
                                   },
                                   &ColorRGB {
                                       red: 5,
                                       green: 5,
                                       blue: 5,
                                   });
    vterm.state_reset(true);
    vterm
}

fn status_line_matches<T: Read>(vterm: &mut VTerm, reader: &mut T, regex: Regex) {
    // what would be cool is if a failure message printed the expectation and the screen buffer
    // contents
    let is_success = ::is_ultimately_true(|| {
        let mut bytes: Vec<u8> = vec![];
        reader.read_to_end(&mut bytes).unwrap();
        vterm.write(&bytes).unwrap();

        // TODO: fix this fixed size by getting size from vterm
        let actual = vterm.screen_get_text_lossy(&Rect {
            start_row: 0,
            end_row: 1,
            start_col: 0,
            end_col: 80,
        });
        regex.is_match(&actual)
    });

    // TODO: fix this fixed size by getting size from vterm
    let actual = vterm.screen_get_text_lossy(&Rect {
        start_row: 0,
        end_row: 5,
        start_col: 0,
        end_col: 10,
    });
    assert!(is_success,
            format!("expected:\n{}\nto match {:#?}", actual, regex));
}

fn send_keys(tx: &Sender<ClientMsg>, bytes: Vec<u8>) {
    tx.send(ClientMsg::UserInput { bytes: bytes }).unwrap();
}

#[test]
fn client_starts_in_welcome_mode() {
    ::setup_logging();
    let mut output = TestIO::new();
    let input = TestIO::new();

    let (_, client) = Client::spawn(input.clone(),
                                    output.clone(),
                                    TtyIoCtlConfig {
                                        rows: 5,
                                        cols: 10,
                                    });
    let mut vterm = build_vterm(&ScreenSize {
        rows: 5,
        cols: 10,
    });
    status_line_matches(&mut vterm, &mut output, Regex::new(r"welcome").unwrap());
    client.stop();
}

#[test]
fn client_can_enter_command_mode() {
    ::setup_logging();
    let mut output = TestIO::new();
    let input = TestIO::new();

    let (tx, client) = Client::spawn(input.clone(),
                                     output.clone(),
                                     TtyIoCtlConfig {
                                         rows: 5,
                                         cols: 10,
                                     });

    let mut vterm = build_vterm(&ScreenSize {
        rows: 5,
        cols: 10,
    });
    send_keys(&tx, vec![b'a']);
    status_line_matches(&mut vterm, &mut output, Regex::new(r"command").unwrap());
    client.stop();
}

#[test]
fn client_can_enter_program_mode() {
    ::setup_logging();
    let mut output = TestIO::new();
    let input = TestIO::new();

    let (tx, client) = Client::spawn(input.clone(),
                                     output.clone(),
                                     TtyIoCtlConfig {
                                         rows: 24,
                                         cols: 80,
                                     });

    let mut vterm = build_vterm(&ScreenSize {
        rows: 24,
        cols: 80,
    });

    send_keys(&tx, vec![b'a']);
    status_line_matches(&mut vterm, &mut output, Regex::new(r"command").unwrap());

    send_keys(&tx, vec![b'c', b'i']);
    // sending this message is hacky
    tx.send(ClientMsg::ProgramAdd {
          server_id: "some server".to_string(),
          program_id: "123".to_string(),
      })
      .unwrap();

    status_line_matches(&mut vterm, &mut output, Regex::new(r"program").unwrap());
    client.stop();
}

#[test]
fn client_can_exit_program_mode() {
    ::setup_logging();
    let mut output = TestIO::new();
    let input = TestIO::new();

    let (tx, client) = Client::spawn(input.clone(),
                                     output.clone(),
                                     TtyIoCtlConfig {
                                         rows: 24,
                                         cols: 80,
                                     });

    // The screen size here is hard coded through the client code. Need to fix that.
    let mut vterm = build_vterm(&ScreenSize {
        rows: 24,
        cols: 80,
    });

    send_keys(&tx, vec![b'a']);
    status_line_matches(&mut vterm, &mut output, Regex::new(r"command").unwrap());

    send_keys(&tx, vec![b'c', b'i']);
    // sending this message is hacky
    tx.send(ClientMsg::ProgramAdd {
          server_id: "some server".to_string(),
          program_id: "123".to_string(),
      })
      .unwrap();
    status_line_matches(&mut vterm, &mut output, Regex::new(r"program").unwrap());

    send_keys(&tx, vec![CTRL_B, b'c']);
    status_line_matches(&mut vterm, &mut output, Regex::new(r"command").unwrap());

    client.stop();
}
