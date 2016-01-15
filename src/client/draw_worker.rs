use std::io;
use std::io::prelude::*;
use std::thread;
use std::sync::mpsc::*;
use std::sync::{Arc, RwLock};
use super::*;
use super::tty_painter::*;
use super::layout::*;
use vterm_sys;

type Cell = vterm_sys::ScreenCell;

/// # todos
/// * [ ] make message enum more specific
/// * [ ] Maybe build with initial state to avoid bootstrap data race?
/// * [ ] optimize drawing when pane is full width (csr)
/// * [ ] implement move cursor
pub struct DrawWorker<F: 'static + Write + Send> {
    rx: Receiver<ClientMsg>,
    painter: TtyPainter<F>,
    layout: Arc<RwLock<Layout>>,
}

impl <F: 'static + Write + Send> DrawWorker<F> {
    pub fn spawn(io: F, rx: Receiver<ClientMsg>, layout: Arc<RwLock<Layout>>) -> thread::JoinHandle<()> {
        info!("spawning draw worker");
        thread::spawn(move || {
            let mut worker = DrawWorker::new(rx, io, layout);
            worker.enter_listen_loop();
            info!("exiting draw worker");
        })
    }

    fn new(rx: Receiver<ClientMsg>, io: F, layout: Arc<RwLock<Layout>>) -> DrawWorker<F> {
        DrawWorker {
            rx: rx,
            painter: TtyPainter::new(io),
            layout: layout
        }
    }

    /// Start receiving messages from Receiver. Exits on a Quit message.
    fn enter_listen_loop(&mut self) {
        loop {
            let msg = match self.rx.recv() {
                Ok(msg) => msg,
                Err(_) => break,
            };

            match msg {
                ClientMsg::Quit => break,
                ClientMsg::ProgramDamage { program_id, cells } => self.program_damage(program_id, cells),
                ClientMsg::LayoutDamage => self.layout_damage(),
                ClientMsg::ProgramMoveCursor { program_id, old, new, is_visible } => self.move_cursor(program_id, new, is_visible),
                _ => warn!("unhandled msg {:?}", msg)
            }
        }
    }

    fn program_damage(&mut self, program_id: String, cells: Vec<vterm_sys::ScreenCell>) {
        trace!("program_damage for {}", program_id);

        let layout = self.layout.read().unwrap();
        if let Some(node) = layout.root.descendants().find(|n| n.is_leaf() && n.value == program_id) {
            self.painter.draw_cells(&cells, &node.computed_pos);
        }
        else {
            warn!("didnt find node with value: {:?}", program_id);
        }
    }

    fn layout_damage(&mut self) {
        trace!("layout_damage");

        let layout = self.layout.read().unwrap();

        let mut cells: Vec<Cell> = vec![];

        self.cells_for_node(&mut cells, &layout.root);
        for node in layout.root.descendants() {
            self.cells_for_node(&mut cells, node);
        }

        self.painter.draw_cells(&cells, &Pos { row: 0, col: 0 });
    }

    fn cells_for_node(&self, cells: &mut Vec<Cell>, node: &Node) {
        if !node.has_border {
            return;
        }

        let distance = node.padding + 1;
        let top      = (node.computed_pos.row - distance as i16);
        let bottom   = (node.computed_pos.row + node.computed_size.rows as i16 - 1 + distance as i16);
        let left     = (node.computed_pos.col - distance as i16);
        let right    = (node.computed_pos.col + node.computed_size.cols as i16 - 1 + distance as i16);

        cells.push(Cell { pos: Pos { row: top, col: left }, chars: vec!['┌'], ..Default::default()});
        cells.push(Cell { pos: Pos { row: top, col: right }, chars: vec!['┐'], ..Default::default()});
        cells.push(Cell { pos: Pos { row: bottom, col: left }, chars: vec!['└'], ..Default::default()});
        cells.push(Cell { pos: Pos { row: bottom, col: right }, chars: vec!['┘'], ..Default::default()});

        for x in (left + 1..right) {
            cells.push(Cell { pos: Pos { row: top, col: x }, chars: vec!['─'], ..Default::default()});
            cells.push(Cell { pos: Pos { row: bottom, col: x }, chars: vec!['─'], ..Default::default()});
        }

        for y in (top + 1..bottom) {
            cells.push(Cell { pos: Pos { row: y, col: left }, chars: vec!['│'], ..Default::default()});
            cells.push(Cell { pos: Pos { row: y, col: right }, chars: vec!['│'], ..Default::default()});
        }
    }

    fn move_cursor(&mut self, program_id: String, pos: vterm_sys::Pos, is_visible: bool) {
        trace!("move_cursor for program {}", program_id);
        // find offset from state
        // painter.move_cursor(pos, is_visible));
    }
}
