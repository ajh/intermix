use std::io::prelude::*;
use super::paint::*;
use std::sync::mpsc::*;
use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};
use super::*;
use super::layout::*;
use super::servers::*;
use uuid::Uuid;
use vterm_sys::{self, Pos, Size, Rect, ScreenCell, RectAssist};

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
    pub layout: Arc<RwLock<layout::Screen>>,
    selected_program_id: Option<String>,
    painter: TtyPainter<F>,
}

static STATUS_LINE: &'static str = "status_line";

impl<F: 'static + Write + Send> MainWorker<F> {
    pub fn spawn(tty_ioctl_config: TtyIoCtlConfig,
                 io: F)
                 -> (Sender<ClientMsg>,
                     Arc<RwLock<layout::Screen>>,
                     JoinHandle<()>) {
        let (tx, rx) = channel::<ClientMsg>();
        let layout = Arc::new(RwLock::new(layout::Screen::new(Size {
            height: tty_ioctl_config.rows,
            width: tty_ioctl_config.cols,
        })));
        let layout_clone = layout.clone();
        let mut worker = MainWorker::new(rx, tx.clone(), tty_ioctl_config, layout, io);

        info!("spawning main worker");
        let handle = thread::spawn(move || {
            worker.enter_listener_loop();
            info!("exiting main worker");
        });

        (tx, layout_clone, handle)
    }

    fn new(rx: Receiver<ClientMsg>,
           tx: Sender<ClientMsg>,
           tty_ioctl_config: TtyIoCtlConfig,
           layout: Arc<RwLock<layout::Screen>>,
           io: F)
           -> MainWorker<F> {

        let size = { layout.read().unwrap().size.clone() };
        let mut painter = TtyPainter::new(io, size);

        let mut worker = MainWorker {
            rx: rx,
            tx: tx,
            servers: Default::default(),
            modal_key_handler: modal::ModalKeyHandler::new_with_graph(),
            tty_ioctl_config: tty_ioctl_config.clone(),
            layout: layout,
            selected_program_id: None,
            painter: painter,
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

        {
            let mut layout = self.layout.write().unwrap();
            layout.tree_mut().root_mut().append(status_line);
            layout.flush_changes();
        }

        self.tx.send(ClientMsg::Clear).unwrap();
        self.tx.send(ClientMsg::LayoutDamage).unwrap();
        self.damage_status_line();
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
                ClientMsg::LayoutSwap { layout } => self.layout_swap(layout),
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
            let mut layout = self.layout.write().unwrap();
            for mut wrap in layout.tree_mut().values_mut() {
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
            layout.flush_changes();
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
        let mut layout = self.layout.write().unwrap();
        layout.tree_mut().root_mut().append(wrap);
        layout.flush_changes();

        self.tx.send(ClientMsg::LayoutDamage).unwrap();
    }

    fn damage_status_line(&self) {
        trace!("damage_status_line for mode {:?}",
               self.modal_key_handler.mode_name());

        let found_status_line = {
            let layout = self.layout.read().unwrap();
            layout.tree().values().find(|n| *n.name() == STATUS_LINE.to_string()).is_some()
        };

        if !found_status_line {
            warn!("no status line node");
            return;
        }

        // Draw it
        let mut cells = vec![];
        for ch in self.modal_key_handler.mode_name().chars() {
            let mut sigh = String::new();
            sigh.push(ch);

            cells.push(vterm_sys::ScreenCell {
                chars: sigh.into_bytes(),
                width: 1,
                attrs: Default::default(),
                fg_palette: 7,
                bg_palette: 0,
                ..Default::default()
            });
        }

        let rect = Rect::new(Pos::new(0,0), Size::new(cells.len(), 1));
        // Does this make sense? A status line is not a program.
        self.tx
            .send(ClientMsg::ProgramDamage {
                program_id: STATUS_LINE.to_string(),
                cells: cells,
                rect: rect,
            })
            .unwrap();
    }

    fn mode_change(&mut self, _: &str) {
        self.damage_status_line();
    }

    fn leaf_names(&self) -> Vec<String> {
        self.layout
            .read()
            .unwrap()
            .tree()
            .values()
            .map(|w| w.name().clone())
            .filter(|n| *n != "root".to_string() && *n != STATUS_LINE.to_string())
            .collect()
    }

    fn layout_swap(&mut self, layout: Arc<RwLock<layout::Screen>>) {
        self.layout = layout;
    }

    fn program_damage(&mut self, program_id: String, cells: Vec<vterm_sys::ScreenCell>, rect: vterm_sys::Rect) {
        trace!("program_damage for {}", program_id);

        let layout = self.layout.read().unwrap();
        if let Some(wrap) = layout.tree().values().find(|w| *w.name() == program_id) {
            let rect = rect.translate(&Pos { x: wrap.computed_x().unwrap(), y: wrap.computed_y().unwrap() });
            self.painter.draw_cells(&cells, &rect);
        } else {
            warn!("didnt find node with value: {:?}", program_id);
        }
    }

    fn clear(&mut self) {
        let layout = self.layout.read().unwrap();
        let mut cells: Vec<ScreenCell> = vec![];

        let rect = Rect::new(Pos::new(0,0), layout.size.clone());

        for pos in rect.positions() {
            cells.push(ScreenCell {
                chars: vec![b' '],
                ..Default::default()
            });
        }

        self.painter.draw_cells(&cells, &rect);
    }

    fn layout_damage(&mut self) {
        trace!("layout_damage");

        let layout = self.layout.read().unwrap();
        // trace!("{:#?}", layout.tree());

        for wrap in layout.tree().values() {
            MainWorker::draw_border_for_node(&mut self.painter, wrap, &layout.size);
        }
    }

    /// TODO: optimize this by batching draw_cells calls
    fn draw_border_for_node(painter: &mut TtyPainter<F>,
                             wrap: &layout::Wrap,
                             size: &Size) {
        if wrap.has_border() {
            let mut top = wrap.border_y().unwrap();
            if top < 0 {
                top = 0
            }

            let mut bottom = wrap.border_y().unwrap() + wrap.border_height().unwrap() - 1;
            if bottom >= size.height {
                bottom = size.height - 1
            }

            let mut left = wrap.border_x().unwrap();
            if left < 0 {
                left = 0
            }

            let mut right = wrap.border_x().unwrap() + wrap.border_width().unwrap() - 1;
            if right >= size.width {
                right = size.width - 1
            }

            painter.draw_cells(&vec![
                                    ScreenCell {
                                        chars: "┌".to_string().into_bytes(),
                                        ..Default::default()
                                    }],
                                    &Rect::new(Pos::new(left, top), Size::new(1, 1))
                            );
            painter.draw_cells(&vec![
                                    ScreenCell {
                                        chars: "┐".to_string().into_bytes(),
                                        ..Default::default()
                                    }],
                                    &Rect::new(Pos::new(right, top), Size::new(1,1))
                                    );
            painter.draw_cells(&vec![
            ScreenCell {
                chars: "└".to_string().into_bytes(),
                ..Default::default()
            }],
                &Rect::new(Pos::new(left, bottom), Size::new(1,1))
                );
            painter.draw_cells(&vec![
            ScreenCell {
                chars: "┘".to_string().into_bytes(),
                ..Default::default()
            }],
                &Rect::new(Pos::new(right,bottom), Size::new(1,1))
            );

            for x in left + 1..right {
                painter.draw_cells(&vec![
                ScreenCell {
                    chars: "─".to_string().into_bytes(),
                    ..Default::default()
                }],
                    &Rect::new(Pos::new(x, top), Size::new(1,1))
                    );
                painter.draw_cells(&vec![
                ScreenCell {
                    chars: "─".to_string().into_bytes(),
                    ..Default::default()
                }],
                    &Rect::new(Pos::new(x, bottom), Size::new(1,1))
                    );
            }

            for y in top + 1..bottom {
                painter.draw_cells(&vec![
                ScreenCell {
                    chars: "│".to_string().into_bytes(),
                    ..Default::default()
                }],
                    &Rect::new(Pos::new(left, y), Size::new(1,1))
                    );
                painter.draw_cells(&vec![
                ScreenCell {
                    chars: "│".to_string().into_bytes(),
                    ..Default::default()
                }],
                    &Rect::new(Pos::new(right, y), Size::new(1,1))
                    );
            }
        } else if wrap.margin() > 0 {
            let mut top = wrap.computed_y().unwrap() - wrap.padding() - 1;
            if top < 0 {
                top = 0
            }

            let mut bottom = wrap.computed_y().unwrap() + wrap.computed_height().unwrap() +
                             wrap.padding();
            if bottom >= size.height {
                bottom = size.height - 1
            }

            let mut left = wrap.computed_x().unwrap() - wrap.padding() - 1;
            if left < 0 {
                left = 0
            }

            let mut right = wrap.computed_x().unwrap() + wrap.computed_width().unwrap() +
                            wrap.padding();
            if right >= size.width {
                right = size.width - 1
            }

            for x in left..right + 1 {
                painter.draw_cells(&vec![
                ScreenCell {
                    chars: " ".to_string().into_bytes(),
                    ..Default::default()
                }],
                    &Rect::new(Pos::new(x, top), Size::new(1,1))
                    );
                painter.draw_cells(&vec![
                ScreenCell {
                    chars: " ".to_string().into_bytes(),
                    ..Default::default()
                }],
                    &Rect::new(Pos::new(x, bottom), Size::new(1,1))
                    );
            }

            for y in top + 1..bottom {
            painter.draw_cells(&vec![
                ScreenCell {
                    chars: " ".to_string().into_bytes(),
                    ..Default::default()
                }],
                    &Rect::new(Pos::new(left, y), Size::new(1,1))
                    );
            painter.draw_cells(&vec![
                ScreenCell {
                    chars: " ".to_string().into_bytes(),
                    ..Default::default()
                }],
                    &Rect::new(Pos::new(right, y), Size::new(1,1))
                    );
            }
        }
    }

    fn move_cursor(&mut self, program_id: String, pos: vterm_sys::Pos, is_visible: bool) {
        let layout = self.layout.read().unwrap();
        if let Some(wrap) = layout.tree().values().find(|w| *w.name() == program_id) {
            let pos = Pos::new(
                pos.x + wrap.computed_x().unwrap(),
                pos.y + wrap.computed_y().unwrap());
            self.painter.move_cursor(pos, is_visible);
        } else {
            warn!("didnt find node with value: {:?}", program_id);
        }
    }
}
