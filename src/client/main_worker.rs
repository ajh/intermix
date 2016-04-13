use std::io::prelude::*;
use super::paint::*;
use std::ops::IndexMut;
use std::sync::mpsc::*;
use std::thread::{self, JoinHandle};
use super::*;
use super::servers::*;
use uuid::Uuid;
use vterm_sys::{self, Pos, Size, Rect, RectAssist};
use ::cell_buffer::*;

/// This worker handles:
/// * user input
/// * server state change messages
///
/// It also owns the client state. When client state is changed it sends msgs to sync the draw
/// worker's internal representation.
///
/// It doesn't receive any server damage messages.
pub struct MainWorker<F: 'static + Write + Send> {
    tx: Sender<ClientMsg>,
    rx: Receiver<ClientMsg>,
    pub servers: Servers,
    pub modal_key_handler: modal::ModalKeyHandler,
    pub tty_ioctl_config: TtyIoCtlConfig,
    pub layout: layout::Screen,
    selected_program_id: Option<String>,
    painter: TtyPainter<F>,
    screen: CellBuffer,
}

static STATUS_LINE: &'static str = "status_line";

impl<F: 'static + Write + Send> MainWorker<F> {
    pub fn spawn(tty_ioctl_config: TtyIoCtlConfig,
                 io: F)
                 -> (Sender<ClientMsg>, JoinHandle<()>) {
        let (tx, rx) = channel::<ClientMsg>();
        let layout = layout::Screen::new(Size::new(tty_ioctl_config.cols, tty_ioctl_config.rows));
        let mut worker = MainWorker::new(rx, tx.clone(), tty_ioctl_config, layout, io);

        info!("spawning main worker");
        let handle = thread::spawn(move || {
            worker.enter_listener_loop();
            info!("exiting main worker");
        });

        (tx, handle)
    }

    fn new(rx: Receiver<ClientMsg>,
           tx: Sender<ClientMsg>,
           tty_ioctl_config: TtyIoCtlConfig,
           layout: layout::Screen,
           io: F)
           -> MainWorker<F> {

        let size = {
            layout.size.clone()
        };

        let mut worker = MainWorker {
            rx: rx,
            tx: tx,
            servers: Default::default(),
            modal_key_handler: modal::ModalKeyHandler::new_with_graph(),
            tty_ioctl_config: tty_ioctl_config.clone(),
            layout: layout,
            selected_program_id: None,
            painter: TtyPainter::new(io, size.clone()),
            screen: CellBuffer::new(size),
        };
        worker.init();
        worker
    }

    /// creates an initial window, status pane etc
    fn init(&mut self) {
        let status_line = layout::WrapBuilder::row()
                              .name(STATUS_LINE.to_string())
                              .height(1)
                              .build();
        self.layout.tree_mut().root_mut().append(status_line);
        self.layout.flush_changes();

        self.tx.send(ClientMsg::Clear).unwrap();
        self.tx.send(ClientMsg::LayoutDamage).unwrap();
        self.tx.send(ClientMsg::StatusLineDamage).unwrap();
    }

    /// Start receiving messages from Receiver. Exits on a Quit message.
    fn enter_listener_loop(&mut self) {
        'outer: loop {
            let msg = match self.rx.recv() {
                Ok(msg) => msg,
                Err(_) => break,
            };

            match msg {
                ClientMsg::Quit => {
                    self.quit();
                    break;
                }
                ClientMsg::ServerAdd { server } => self.servers.add_server(server),
                ClientMsg::ProgramAdd { server_id, program_id } => {
                    self.add_program(server_id, program_id)
                }
                ClientMsg::ProgramDamage { program_id, cells, rect } => {
                    self.program_damage(program_id, cells, rect)
                }
                ClientMsg::Clear { .. } => self.clear(),
                ClientMsg::LayoutDamage { .. } => self.layout_damage(),
                ClientMsg::ProgramMoveCursor { program_id, old: _, new, is_visible } => {
                    self.move_cursor(program_id, new, is_visible)
                }
                ClientMsg::LayoutSwap { layout } => self.layout = layout,
                ClientMsg::StatusLineDamage => self.damage_status_line(),
                ClientMsg::UserInput { bytes } => {
                    self.modal_key_handler.write(&bytes).unwrap();
                    while let Some(user_action) = self.modal_key_handler.actions_queue.pop() {
                        match user_action {
                            modal::UserAction::ModeChange { name } => self.mode_change(&name),
                            modal::UserAction::ProgramFocus => self.program_focus_cmd(),
                            modal::UserAction::ProgramInput { bytes: fites } => {
                                self.program_input_cmd(fites)
                            }
                            modal::UserAction::ProgramStart => self.program_start_cmd(),
                            modal::UserAction::ProgramSelectPrev => self.program_select_prev(),
                            modal::UserAction::ProgramSelectNext => self.program_select_next(),
                            modal::UserAction::Quit => {
                                self.quit();
                                break 'outer;
                            }
                            modal::UserAction::UnknownInput { bytes: fites } => {
                                error!("unknown input for mode {}: {:?}",
                                       self.modal_key_handler.mode_name(),
                                       fites)
                            }
                        }
                    }
                }
                _ => warn!("unhandled msg {:?}", msg),
            }
        }
    }

    fn quit(&self) {
        info!("quit!");
        for server in self.servers.iter() {
            server.tx.send(::server::ServerMsg::Quit).unwrap();
        }
    }

    fn program_input_cmd(&self, bytes: Vec<u8>) {
        if let Some(program_id) = self.selected_program_id.clone() {
            if let Some(server) = self.servers
                                      .iter()
                                      .find(|s| s.programs.iter().any(|p| p.id == program_id)) {
                trace!("sending input to program {} {:?}", program_id, bytes);
                server.tx
                      .send(::server::ServerMsg::ProgramInput {
                          program_id: program_id,
                          bytes: bytes,
                      })
                      .unwrap();
            } else {
                warn!("server doesn't have a program called {:?}", program_id);
            }
        } else {
            warn!("program input without selected program");
        }
    }

    fn program_start_cmd(&self) {
        if let Some(server) = self.servers.first() {
            trace!("starting program");
            // for now, always start bash
            let command_and_args: Vec<String> = vec!["bash".to_string()];
            server.tx
                  .send(::server::ServerMsg::ProgramStart {
                      command_and_args: command_and_args,
                      program_id: Uuid::new_v4().to_hyphenated_string(),
                  })
                  .unwrap();
        }
    }

    /// The main point of the command, which is to direct user keys to the program, has already
    /// been done by the modal state machine. All we have to do is make sure a program is selected.
    fn program_focus_cmd(&mut self) {
        trace!("program_focus_cmd {:?}", self.selected_program_id);

        let valid_selection = if let Some(program_id) = self.selected_program_id.clone() {
            if self.leaf_names().iter().any(|n| *n == program_id) {
                true
            } else {
                false
            }
        } else {
            false
        };

        if !valid_selection {
            self.program_select_next();
        }
    }

    fn program_select_prev(&mut self) {
        let mut selected_index = if let Some(program_id) = self.selected_program_id.clone() {
            if let Some(i) = self.leaf_names().iter().position(|n| *n == program_id) {
                i
            } else {
                0
            }
        } else {
            0
        };

        if selected_index > 0 {
            selected_index -= 1;
        }

        self.selected_program_id = Some(self.leaf_names()[selected_index].clone());
        self.add_border_to_selected_program_id_wrap();
    }

    fn program_select_next(&mut self) {
        let leaf_names: Vec<String> = self.leaf_names();

        if let Some(program_id) = self.selected_program_id.clone() {
            if let Some(mut i) = leaf_names.iter().position(|n| *n == program_id) {
                i += 1;
                if i < leaf_names.len() {
                    self.selected_program_id = Some(leaf_names[i].clone());
                } else {
                    self.selected_program_id = Some(leaf_names[0].clone());
                }
            } else {
                self.selected_program_id = None;
            }
        }

        if self.selected_program_id.is_none() && leaf_names.len() > 0 {
            self.selected_program_id = Some(leaf_names[0].clone());
        }

        self.add_border_to_selected_program_id_wrap();
    }

    fn add_border_to_selected_program_id_wrap(&mut self) {
        if let Some(program_id) = self.selected_program_id.clone() {
            for mut wrap in self.layout.tree_mut().values_mut() {
                if *wrap.name() == "root".to_string() {
                    continue;
                }
                if *wrap.name() == STATUS_LINE {
                    continue;
                }
                if *wrap.name() == program_id {
                    wrap.set_has_border(true);
                    wrap.set_margin(0);
                } else {
                    wrap.set_has_border(false);
                    wrap.set_margin(1);
                }
            }
            self.layout.flush_changes();
        }

        self.tx.send(ClientMsg::LayoutDamage).unwrap();
    }

    fn add_program(&mut self, server_id: String, program_id: String) {
        self.servers.add_program(&server_id,
                                 Program {
                                     id: program_id.clone(),
                                     is_subscribed: true,
                                 });

        let wrap = layout::WrapBuilder::row()
                       .name(program_id.clone())
                       .height(24)
                       .width(80)
                       .margin(1)
                       .build();
        self.layout.tree_mut().root_mut().append(wrap);
        self.layout.flush_changes();

        self.tx.send(ClientMsg::LayoutDamage).unwrap();
    }

    fn damage_status_line(&mut self) {
        trace!("damage_status_line for mode {:?}",
               self.modal_key_handler.mode_name());

        if let Some(wrap) = self.layout.tree().values().find(|n| *n.name() == STATUS_LINE.to_string()) {
            let rect = Rect::new(Pos::new(wrap.computed_x().unwrap(), wrap.computed_y().unwrap()),
                                 Size::new(wrap.computed_width().unwrap(),
                                           wrap.computed_height().unwrap()));

            for pos in rect.positions() {
                let cell = self.screen.index_mut(pos);
                cell.dirty = true;
                cell.chars.clear();
            }

            for (pos, ch) in rect.positions().zip(self.modal_key_handler.mode_name().chars()) {
                // TODO: find a better way to convert from a char to Vec<u8>. Maybe encode_utf8?
                let mut sigh = String::new();
                sigh.push(ch);
                let cell = self.screen.index_mut(pos);
                cell.chars = sigh.into_bytes();
                cell.dirty = true;
            }

            self.painter.draw_screen(&mut self.screen);
        } else {
            warn!("no status line node");
        }
    }

    fn mode_change(&mut self, _: &str) {
        self.damage_status_line();
    }

    fn leaf_names(&self) -> Vec<String> {
        self.layout
            .tree()
            .values()
            .map(|w| w.name().clone())
            .filter(|n| *n != "root".to_string() && *n != STATUS_LINE.to_string())
            .collect()
    }

    fn program_damage(&mut self,
                      program_id: String,
                      cells: Vec<vterm_sys::ScreenCell>,
                      rect: vterm_sys::Rect) {
        trace!("program_damage for {}", program_id);

        if let Some(wrap) = self.layout.tree().values().find(|w| *w.name() == program_id) {
            for (vterm_cell, pos) in cells.iter().zip(rect.positions()) {
                let pos = pos + Pos::new(wrap.computed_x().unwrap(), wrap.computed_y().unwrap());
                let mut cell = self.screen.index_mut(pos);

                // TODO: make the wire data format be the same as Cell so this is just a memcopy
                // into a vector
                cell.update_from_vterm_cell(&vterm_cell);
                cell.dirty = true;
            }

            self.painter.draw_screen(&mut self.screen);
        } else {
            warn!("didnt find node with value: {:?}", program_id);
        }
    }

    fn clear(&mut self) {
        for pair in self.screen.iter_mut() {
            pair.0.clear();
        }

        self.painter.draw_screen(&mut self.screen);
    }

    fn layout_damage(&mut self) {
        trace!("layout_damage");

        for wrap in self.layout.tree().values() {
            MainWorker::draw_node_box(&mut self.screen, wrap, &mut self.painter, &self.layout.size);
        }
    }

    /// Draw any margin border or padding for the given node
    fn draw_node_box(screen: &mut CellBuffer,
                     wrap: &layout::Wrap,
                     painter: &mut TtyPainter<F>,
                     size: &Size) {
        let screen_rect = Rect::new(Pos::new(0, 0), size.clone());

        let outside_rect = Rect::new(Pos::new(wrap.outside_x().unwrap(),
                                              wrap.outside_y().unwrap()),
                                     Size::new(wrap.outside_width().unwrap(),
                                               wrap.outside_height().unwrap()));
        let outside_rect = outside_rect.intersection(&screen_rect).unwrap();

        let inside_rect = Rect::new(Pos::new(wrap.computed_x().unwrap(),
                                             wrap.computed_y().unwrap()),
                                    Size::new(wrap.computed_width().unwrap(),
                                              wrap.computed_height().unwrap()));
        let inside_rect = inside_rect.intersection(&screen_rect).unwrap();

        for pos in outside_rect.positions().filter(|p| !inside_rect.contains(p)) {
            screen.index_mut(pos).clear();
        }

        if wrap.has_border() {
            let border_rect = Rect::new(Pos::new(wrap.border_x().unwrap(),
                                                 wrap.border_y().unwrap()),
                                        Size::new(wrap.border_width().unwrap(),
                                                  wrap.border_height().unwrap()));
            let border_rect = border_rect.intersection(&screen_rect).unwrap();

            let top_and_bottoms = border_rect.positions().filter(|p| {
                p.y == border_rect.min_y() || p.y == border_rect.max_y() - 1
            });
            for pos in top_and_bottoms {
                screen.index_mut(pos).chars = "─".to_string().into_bytes();
            }

            let left_and_rights = border_rect.positions().filter(|p| {
                p.x == border_rect.min_x() || p.x == border_rect.max_x() - 1
            });
            for pos in left_and_rights {
                screen.index_mut(pos).chars = "│".to_string().into_bytes();
            }

            screen.index_mut(border_rect.origin).chars = "┌".to_string().into_bytes();
            screen.index_mut(Pos::new(border_rect.origin.x + border_rect.size.width - 1,
                                      border_rect.origin.y))
                  .chars = "┐".to_string().into_bytes();
            screen.index_mut(Pos::new(border_rect.origin.x,
                                      border_rect.origin.y + border_rect.size.height - 1))
                  .chars = "└".to_string().into_bytes();
            screen.index_mut(Pos::new(border_rect.origin.x + border_rect.size.width - 1,
                                      border_rect.origin.y + border_rect.size.height - 1))
                  .chars = "┘".to_string().into_bytes();
        }

        painter.draw_screen(screen);
    }

    fn move_cursor(&mut self, program_id: String, pos: vterm_sys::Pos, is_visible: bool) {
        if let Some(wrap) = self.layout.tree().values().find(|w| *w.name() == program_id) {
            let pos = Pos::new(pos.x + wrap.computed_x().unwrap(),
                               pos.y + wrap.computed_y().unwrap());
            self.painter.move_cursor(pos, is_visible);
        } else {
            warn!("didnt find node with value: {:?}", program_id);
        }
    }
}
