use std::io::prelude::*;
use std::sync::{Arc, RwLock};
use libintermix::client::*;
use vterm_sys::*;
use ::support::test_io::*;
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
// * [ ] compare vterms
// * [ ] refactor
// * [ ] downsize screen buffers
//

#[test]
fn it_runs_thingy_bobby() {
    ::setup_logging();
    // create client with simplified layout
    let tty_ioctl_config = TtyIoCtlConfig {
        rows: 24,
        cols: 80,
        ..Default::default()
    };

    let test_output = ::support::test_io::TestIO::new();
    let (client_tx, client) = Client::spawn(::std::io::empty(),
                                            test_output.clone(),
                                            tty_ioctl_config);

    let mut layout = layout::Screen::new(ScreenSize {
        rows: 24,
        cols: 80,
    });
    let leaf = layout::WrapBuilder::row()
                   .name("test_program".to_string())
                   .width(80)
                   .height(24)
                   .build();
    layout.tree_mut().root_mut().append(leaf);
    layout.flush_changes();
    client_tx.send(ClientMsg::LayoutSwap { layout: Arc::new(RwLock::new(layout)) }).unwrap();

    // start ttyplay
    let handle = thread::spawn(move || {
        let p = env::current_dir().unwrap();
        info!("running echo");
        let output = Command::new("echo")
                             .arg(p.join("hi"))
                             .output()
                             .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
        let mut vterm = VTerm::new(ScreenSize {
            rows: 24,
            cols: 80,
        });
        vterm.set_utf8(true);
        vterm.screen_set_damage_merge(ffi::VTermDamageSize::VTermDamageRow);
        let vterm_event_rx = vterm.receive_screen_events();
        info!("writing ttyplay output to vterm");
        vterm.write(output.stdout.as_slice());
        vterm.screen_flush_damage();

        info!("reading vterm msgs");
        loop {
            match vterm_event_rx.try_recv() {
                Ok(event) => {
                    match event {
                        ScreenEvent::Bell => info!("Bell"),
                        ScreenEvent::Damage{rect} => {
                            info!("Damage: rect={:?}", rect);
                            let event = ClientMsg::ProgramDamage {
                                program_id: "test_program".to_string(),
                                cells: vterm.screen_get_cells_in_rect(&rect),
                            };
                            client_tx.send(event).unwrap();
                        }
                        ScreenEvent::MoveCursor{new, old, is_visible} => {
                            info!("MoveCursor: new={:?} old={:?} is_visible={:?}", new, old, is_visible);
                            let event = ClientMsg::ProgramMoveCursor {
                                program_id: "test_program".to_string(),
                                new: new,
                                old: old,
                                is_visible: is_visible,
                            };
                            client_tx.send(event).unwrap();
                        },
                        ScreenEvent::MoveRect{dest, src} => {
                            info!("MoveRect: dest={:?} src={:?}", dest, src);
                            let event = ClientMsg::ProgramDamage {
                                program_id: "test_program".to_string(),
                                cells: vterm.screen_get_cells_in_rect(&dest),
                            };
                            client_tx.send(event).unwrap();
                        }
                        ScreenEvent::Resize{rows, cols} => info!("Resize: rows={:?} cols={:?}", rows, cols),
                        ScreenEvent::SbPopLine{cells: _} => info!("SbPopLine"),
                        ScreenEvent::SbPushLine{cells} => info!("SbPushLine"),
                        ScreenEvent::AltScreen{ is_true } => info!("AltScreen: is_true={:?}", is_true),
                        ScreenEvent::CursorBlink{ is_true } => info!("CursorBlink: is_true={:?}", is_true),
                        ScreenEvent::CursorShape{ value } => info!("CursorShape: value={:?}", value),
                        ScreenEvent::CursorVisible{ is_true } => info!("CursorVisible: is_true={:?}", is_true),
                        ScreenEvent::IconName{ text } => info!("IconName: text={:?}", text),
                        ScreenEvent::Mouse{ value } => info!("Mouse: value={:?}", value),
                        ScreenEvent::Reverse{ is_true } => info!("Reverse: is_true={:?}", is_true),
                        ScreenEvent::Title{ text } => info!("Title: text={:?}", text),
                    }
                },
                Err(..) => break
            }
        }
        info!("done reading vterm msgs");

        // the vterm here has the expected screen buffer
        // return a bunch of strings representing the different states of the screen buffer.
        vterm.screen_get_text(Rect { start_row: 0, end_row: 24, start_col: 0, end_col: 80 })
    });

    let expected = handle.join().unwrap();

    // run intermix output through another vterm
    let mut vterm = VTerm::new(ScreenSize {
        rows: 24,
        cols: 80,
    });
    vterm.state_set_default_colors(ColorRGB { red: 230, green: 230, blue: 230 },
                                   ColorRGB { red: 5, green: 5, blue: 5 });
    vterm.state_reset(true);

    let mut test_output_clone = test_output.clone();
    let expected_clone = expected.clone();
    let result = ::is_ultimately_true_result(move || {
        let mut bytes: Vec<u8> = vec![];
        test_output_clone.read_to_end(&mut bytes).unwrap();
        vterm.write(&bytes);

        // TODO: fix this fixed size by getting size from vterm
        let actual = vterm.screen_get_text(Rect { start_row: 0, end_row: 24, start_col: 0, end_col: 80 });
        if expected_clone == actual {
            Ok(())
        }
        else {
            Err(actual)
        }
    });

    match result {
        Ok(()) => {},
        Err(actual) => {
            println!("expected:\n{}", expected);
            println!("actual:\n{}", actual);
            assert!(false);
        }
    }
}
