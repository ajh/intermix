use std::io::prelude::*;
use libintermix::client::*;
use vterm_sys::*;
use ::support::*;
use std::thread;
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
fn run_command_in_vterm(cmd: CommandBuilder, size: Size) -> VTerm {
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
        vterm.screen_set_damage_merge(DamageSize::Row);

        vterm.screen_receive_events(&ScreenCallbacksConfig::all());
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
                    ScreenEvent::Damage(e) => {
                        info!("Damage: {:?}", e);
                        let event = ClientMsg::ProgramDamage {
                            program_id: "test_program".to_string(),
                            cells: vterm.screen_get_cells_in_rect(&e.rect),
                            rect: e.rect,
                        };
                        client.tx().send(event).unwrap();
                    }
                    ScreenEvent::MoveCursor(e) => {
                        info!("MoveCursor: {:?}", e);
                        let event = ClientMsg::ProgramMoveCursor {
                            program_id: "test_program".to_string(),
                            new: e.new,
                            old: e.old,
                            is_visible: e.is_visible,
                        };
                        client.tx().send(event).unwrap();
                    }
                    ScreenEvent::MoveRect(e) => {
                        info!("MoveRect: {:?}", e);
                        let event = ClientMsg::ProgramDamage {
                            program_id: "test_program".to_string(),
                            cells: vterm.screen_get_cells_in_rect(&e.src),
                            rect: e.src,
                        };
                        client.tx().send(event).unwrap();
                        let event = ClientMsg::ProgramDamage {
                            program_id: "test_program".to_string(),
                            cells: vterm.screen_get_cells_in_rect(&e.dest),
                            rect: e.dest,
                        };
                        client.tx().send(event).unwrap();
                    }
                    ScreenEvent::Resize(e) => info!("Resize: {:?}", e),
                    ScreenEvent::SbPopLine(e) => info!("SbPopLine: {:?}", e),
                    ScreenEvent::SbPushLine(e) => info!("SbPushLine: {:?}", e),
                    ScreenEvent::AltScreen(e) => info!("AltScreen: {:?}", e),
                    ScreenEvent::CursorBlink(e) => info!("CursorBlink: {:?}", e),
                    ScreenEvent::CursorShape(e) => info!("CursorShape: {:?}", e),
                    ScreenEvent::CursorVisible(e) => info!("CursorVisible: {:?}", e),
                    ScreenEvent::IconName(e) => info!("IconName: {:?}", e),
                    ScreenEvent::Mouse(e) => info!("Mouse: {:?}", e),
                    ScreenEvent::Reverse(e) => info!("Reverse: {:?}", e),
                    ScreenEvent::Title(e) => info!("Title: {:?}", e),
                }
            }
            Err(..) => break,
        }
    }
    info!("done reading vterm msgs");
}

/// Build and return a simplified client of the given size.
fn build_client(output: TestIO, size: &Size) -> Client {
    let (client_tx, client) = Client::spawn(::std::io::empty(),
                                            output,
                                            TtyIoCtlConfig {
                                                rows: size.height,
                                                cols: size.width,
                                                ..Default::default()
                                            });

    let mut layout = layout::Screen::new(Size {
        height: size.height,
        width: size.width,
    });

    let leaf = layout::WrapBuilder::row()
                   .name("test_program".to_string())
                   .width(80)
                   .height(24)
                   .build();
    layout.tree_mut().root_mut().append(leaf);
    layout.flush_changes();
    client_tx.send(ClientMsg::LayoutSwap { layout: layout }).unwrap();

    client
}

/// Build a new VTerm with consistent settings
fn build_vterm(size: &Size) -> VTerm {
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

    let size = Size { height: 4, width: 4 };

    let mut expected_vterm: VTerm = run_command_in_vterm(CommandBuilder::new("echo")
                                                             .arg("some stuff"),
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

#[test]
fn it_draws_simple_vim_session() {
    ::setup_logging();

    let size = Size {
        height: 5,
        width: 29,
    };
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

#[test]
fn it_draws_vim_cargo_toml_with_scrolling() {
    ::setup_logging();

    let size = Size {
        height: 5,
        width: 29,
    };
    let mut expected_vterm: VTerm = run_command_in_vterm(CommandBuilder::new("ttyplay")
                                                             .arg(env::current_dir()
                                                                      .unwrap()
                                                                      .join("tests/tty_recordin\
                                                                             gs/vim_cargo_toml.5x32.ttyrec")
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
