#![feature(test)]

extern crate test;
extern crate ego_tree;
extern crate libintermix;
extern crate regex;
extern crate term;
extern crate time;
extern crate vterm_sys;

use std::io::prelude::*;
use libintermix::client::*;
use vterm_sys::*;
use std::thread;
use std::env;
use std::process::Command;
use test::Bencher;

// Runs the given command and returns the expected value which is based on the contents of a vterm
// screen buffer after writing the comands output to it.
fn run_command_in_vterm(size: Size) -> VTerm {
    let handle: thread::JoinHandle<VTerm> = thread::spawn(move || {
        let mut command = Command::new("ttyplay");
        command.args(&[
            env::current_dir()
            .unwrap()
            .join("tests/tty_recordings/vim_cargo_toml.5x32.ttyrec")
            .to_str()
            .unwrap()]);

        let output = command.output()
                        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
        if !output.status.success() {
            panic!("command returned non-zero status code {:?}: {}",
                   output.status.code(),
                   String::from_utf8_lossy(&output.stderr));
        }

        let mut vterm = build_vterm(&size);
        vterm.screen_set_damage_merge(DamageSize::Row);

        vterm.screen_receive_events(&ScreenCallbacksConfig::all());
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
                    ScreenEvent::Bell => {},
                    ScreenEvent::Damage(e) => {
                        let event = ClientMsg::ProgramDamage {
                            program_id: "test_program".to_string(),
                            cells: vterm.screen_get_cells_in_rect(&e.rect),
                            rect: e.rect,
                        };
                        client.tx().send(event).unwrap();
                    }
                    ScreenEvent::MoveCursor(e) => {
                        let event = ClientMsg::ProgramMoveCursor {
                            program_id: "test_program".to_string(),
                            new: e.new,
                            old: e.old,
                            is_visible: e.is_visible,
                        };
                        client.tx().send(event).unwrap();
                    }
                    ScreenEvent::MoveRect(e) => {
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
                    ScreenEvent::Resize(_) => {},
                    ScreenEvent::SbPopLine(_) => {},
                    ScreenEvent::SbPushLine(_) => {},
                    ScreenEvent::AltScreen(_) => {},
                    ScreenEvent::CursorBlink(_) => {},
                    ScreenEvent::CursorShape(_) => {},
                    ScreenEvent::CursorVisible(_) => {},
                    ScreenEvent::IconName(_) => {},
                    ScreenEvent::Mouse(_) => {},
                    ScreenEvent::Reverse(_) => {},
                    ScreenEvent::Title(_) => {},
                }
            }
            Err(..) => break,
        }
    }
}

/// Build and return a simplified client of the given size.
fn build_client(size: &Size) -> Client {
    let (client_tx, client) = Client::spawn(::std::io::empty(),
                                            ::std::io::stderr(),
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

#[bench]
fn draw_vim(b: &mut Bencher) {
    b.iter(|| {
        let size = Size {
            height: 5,
            width: 29,
        };
        let mut expected_vterm: VTerm = run_command_in_vterm(size.clone());
        let mut client = build_client(&size);
        load_vterm_events_into_client(&mut expected_vterm, &mut client);
    });
}
