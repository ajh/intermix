use std::io::prelude::*;
use std::sync::{Arc, RwLock};
use libintermix::client::*;
use vterm_sys::*;
use ::support::*;
use std::thread;
use std::process::Command;
use std::env;

// This is an integration test on the client that tests drawing to the tty.
//
// The testing approach is to:
//
// 1. load typescript files through vterm
// 2. pass messages through client
// 3. read client output back through another vterm
// 4. asset that the vterm from steps #1 and #3 are equal
//
// The ascii file used controls the scenario being tested.
//
// Will try using ttyrec and ttyplay for the ascii escape code data.
//
// TODO:
//
// * [x] client command to load a given layout. Will load a full screen program.
// * [x] build test tool that works like the server vte code, runs ttyplay on the given file, reads
// psuedo terminal output into vterm, and converts vte callback info into client messages
// * [x] pass client messages to client
// * [x] read client output into a vterm
// * [x] wait for everything to finish???
// * [x] compare vterms
// * [x] refactor
// * [x] downsize screen buffers
// * [ ] have client start in some mode were it doesn't print 'welcome'
//

// Runs the given command and returns the expected value which is based on the contents of a vterm
// screen buffer after writing the comands output to it.
fn run_command_in_vterm(cmd: CommandBuilder, size: ScreenSize) -> VTerm {
    let handle: thread::JoinHandle<VTerm> = thread::spawn(move || {
        let output = cmd.build()
                        .output()
                        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
        if !output.status.success() {
            panic!("command returned non-zero status code {:?}: {}",
                   output.status.code(),
                   String::from_utf8_lossy(&output.stderr));
        }

        let mut vterm = build_vterm(&size);
        vterm.screen_set_damage_merge(ffi::VTermDamageSize::VTermDamageRow);

        vterm.generate_screen_events().unwrap();
        info!("writing ttyplay output to vterm");
        vterm.write(output.stdout.as_slice()).unwrap();
        vterm.screen_flush_damage();

        vterm
    });

    handle.join().unwrap()
}

fn load_vterm_events_into_client(vterm: &mut VTerm, client: &mut Client) {
    let rx = vterm.screen_event_rx.take().unwrap();
    loop {
        match rx.try_recv() {
            Ok(event) => {
                match event {
                    ScreenEvent::Bell => info!("Bell"),
                    ScreenEvent::Damage{rect} => {
                        info!("Damage: rect={:?}", rect);
                        let event = ClientMsg::ProgramDamage {
                            program_id: "test_program".to_string(),
                            cells: vterm.screen_get_cells_in_rect(&rect),
                        };
                        client.tx().send(event).unwrap();
                    }
                    ScreenEvent::MoveCursor{new, old, is_visible} => {
                        info!("MoveCursor: new={:?} old={:?} is_visible={:?}",
                              new,
                              old,
                              is_visible);
                        let event = ClientMsg::ProgramMoveCursor {
                            program_id: "test_program".to_string(),
                            new: new,
                            old: old,
                            is_visible: is_visible,
                        };
                        client.tx().send(event).unwrap();
                    }
                    ScreenEvent::MoveRect{dest, src} => {
                        info!("MoveRect: dest={:?} src={:?}", dest, src);
                        let event = ClientMsg::ProgramDamage {
                            program_id: "test_program".to_string(),
                            cells: vterm.screen_get_cells_in_rect(&dest),
                        };
                        client.tx().send(event).unwrap();
                    }
                    ScreenEvent::Resize{rows, cols} => {
                        info!("Resize: rows={:?} cols={:?}", rows, cols)
                    }
                    ScreenEvent::SbPopLine{cells: _} => info!("SbPopLine"),
                    ScreenEvent::SbPushLine{cells: _} => info!("SbPushLine"),
                    ScreenEvent::AltScreen{ is_true } => info!("AltScreen: is_true={:?}", is_true),
                    ScreenEvent::CursorBlink{ is_true } => {
                        info!("CursorBlink: is_true={:?}", is_true)
                    }
                    ScreenEvent::CursorShape{ value } => info!("CursorShape: value={:?}", value),
                    ScreenEvent::CursorVisible{ is_true } => {
                        info!("CursorVisible: is_true={:?}", is_true)
                    }
                    ScreenEvent::IconName{ text } => info!("IconName: text={:?}", text),
                    ScreenEvent::Mouse{ value } => info!("Mouse: value={:?}", value),
                    ScreenEvent::Reverse{ is_true } => info!("Reverse: is_true={:?}", is_true),
                    ScreenEvent::Title{ text } => info!("Title: text={:?}", text),
                }
            }
            Err(..) => break,
        }
    }
    info!("done reading vterm msgs");
}

/// Build and return a simplified client of the given size.
fn build_client(output: TestIO, size: &ScreenSize) -> Client {
    let (client_tx, client) = Client::spawn(::std::io::empty(),
                                            output,
                                            TtyIoCtlConfig {
                                                rows: size.rows,
                                                cols: size.cols,
                                                ..Default::default()
                                            });

    let mut layout = layout::Screen::new(ScreenSize {
        rows: size.rows,
        cols: size.cols,
    });

    let leaf = layout::WrapBuilder::row()
                   .name("test_program".to_string())
                   .width(80)
                   .height(24)
                   .build();
    layout.tree_mut().root_mut().append(leaf);
    layout.flush_changes();
    client_tx.send(ClientMsg::LayoutSwap { layout: Arc::new(RwLock::new(layout)) }).unwrap();

    client
}

/// Build a new VTerm with consistent settings
fn build_vterm(size: &ScreenSize) -> VTerm {
    let mut vterm = VTerm::new(size);
    let fg = vterm.state_get_rgb_color_from_palette(7);
    let bg = vterm.state_get_rgb_color_from_palette(0);
    vterm.state_set_default_colors(&fg, &bg);
    vterm.set_utf8(true);

    vterm.screen_reset(true);

    vterm
}

#[test]
fn it_draws_simple_echo_output() {
    ::setup_logging();

    let size = ScreenSize { rows: 4, cols: 4 };

    let mut expected_vterm: VTerm = run_command_in_vterm(CommandBuilder::new("echo")
                                                             .arg("some stuff"),
                                                         size.clone());
    println!("{:?}", expected_vterm.state_get_default_colors());

    let mut test_output = TestIO::new();
    let mut client = build_client(test_output.clone(), &size);
    load_vterm_events_into_client(&mut expected_vterm, &mut client);

    let mut actual_vterm = build_vterm(&size);
    println!("{:?}", actual_vterm.state_get_default_colors());

    let result = ::try_until_ok(move || {
        let mut bytes: Vec<u8> = vec![];
        test_output.read_to_end(&mut bytes).unwrap();
        actual_vterm.write(&bytes).unwrap();
        let diff = VTermDiff::new(&expected_vterm, &actual_vterm);
        if diff.has_diff() {
            Err(format!("{}", diff))
        } else {
            Ok(())
        }
    });

    match result {
        Ok(()) => {}
        Err(diff) => assert!(false, diff),
    }
}

#[test]
fn it_draws_simple_vim_session() {
    ::setup_logging();

    let size = ScreenSize {
        rows: 5,
        cols: 29,
    };
    let mut cmd = Command::new("ttyplay2");
    cmd.arg(env::current_dir().unwrap().join("tests/tty_recordings/vim.5x29.ttyrec"));
    let mut expected_vterm: VTerm = run_command_in_vterm(CommandBuilder::new("ttyplay")
                                                             .arg(env::current_dir()
                                                                      .unwrap()
                                                                      .join("tests/tty_recordin\
                                                                             gs/vim.5x29.ttyrec")
                                                                      .to_str()
                                                                      .unwrap()),
                                                         size.clone());

    let mut test_output = TestIO::new();
    let mut client = build_client(test_output.clone(), &size);
    load_vterm_events_into_client(&mut expected_vterm, &mut client);

    let mut actual_vterm = build_vterm(&size);

    let result = ::try_until_ok(move || {
        let mut bytes: Vec<u8> = vec![];
        test_output.read_to_end(&mut bytes).unwrap();
        actual_vterm.write(&bytes).unwrap();
        let diff = VTermDiff::new(&expected_vterm, &actual_vterm);
        if diff.has_diff() {
            Err(format!("{}", diff))
        } else {
            Ok(())
        }
    });

    match result {
        Ok(()) => {}
        Err(diff) => assert!(false, diff),
    }
}
