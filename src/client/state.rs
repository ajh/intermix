use vterm_sys;
use std::sync::mpsc::*;
use std::fmt;
use super::modal::*;
use std::slice;

/// Represents the state of the client windows and panes. Each thread will maintain their own
/// representation which will stay in sync through message passing. The canonical version is owned
/// by the main thread.
#[derive(Default, Clone, Debug)]
pub struct Windows {
    windows: Vec<Window>,
}

impl Windows {
    /// Returns an iterator of its windows
    pub fn iter(&self) -> slice::Iter<Window> {
        self.windows.iter()
    }

    /// Returns an iterator of its windows which are mutable
    pub fn iter_mut(&mut self) -> slice::IterMut<Window> {
        self.windows.iter_mut()
    }

    pub fn add_window(&mut self, window: Window) {
        if !self.windows.iter().any(|w| w.id == window.id) {
            trace!("add window {:?}", window.id);
            self.windows.push(window)
        }
    }

    pub fn remove_window(&mut self, id: &str) {
        if let Some(i) = self.windows.iter().position(|w| w.id == id) {
            trace!("remove window {:?}", id);
            self.windows.remove(i);
        }
    }

    pub fn add_pane(&mut self, window_id: &str, pane: Pane) {
        let window = self.windows.iter_mut().find(|w| w.id == window_id);
        if window.is_none() {
            trace!("couldnt add pane {:?} to unknown window {:?}", pane.id, window_id);
            return
        }
        let mut window = window.unwrap();

        if !window.panes.iter().any(|p| p.id == pane.id) {
            trace!("add pane {:?}", pane.id);
            window.panes.push(pane)
        }
    }
}

/// The window that the user sees. Fills the user's tty.
#[derive(Default, Clone, Debug)]
pub struct Window {
    pub id: String,
    pub panes: Vec<Pane>,
    pub size: vterm_sys::ScreenSize,
}

/// A rectange within the window that displays output from a program.
///
/// TODO: It'd be nice to have something to tie back to its window.
#[derive(Default, Clone, Debug)]
pub struct Pane {
    pub id: String,
    pub size: vterm_sys::ScreenSize,
    pub offset: vterm_sys::Pos,
    pub program_id: String,
}

#[derive(Default, Clone, Debug)]
pub struct Servers {
    servers: Vec<Server>,
}

impl Servers {
    pub fn iter(&self) -> slice::Iter<Server> {
        self.servers.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<Server> {
        self.servers.iter_mut()
    }

    pub fn first(&self) -> Option<&Server> {
        self.servers.first()
    }

    pub fn add_server(&mut self, server: Server) {
        if !self.servers.iter().any(|w| w.id == server.id) {
            trace!("add server {:?}", server.id);
            self.servers.push(server)
        }
    }

    pub fn remove_server(&mut self, id: &str) {
        if let Some(i) = self.servers.iter().position(|w| w.id == id) {
            trace!("remove server {:?}", id);
            self.servers.remove(i);
        }
    }

    pub fn add_program(&mut self, server_id: &str, program: Program) {
        let server = self.servers.iter_mut().find(|w| w.id == server_id);
        if server.is_none() {
            trace!("couldnt add program {:?} to unknown server {:?}", program.id, server_id);
            return
        }
        let mut server = server.unwrap();

        if !server.programs.iter().any(|p| p.id == program.id) {
            trace!("add program {:?}", program.id);
            server.programs.push(program)
        }
    }
}

/// A connection to an intermix server
#[derive(Clone)]
pub struct Server {
    pub id: String,
    pub programs: Vec<Program>,

    /// replace with with cap'n proto or whatever
    pub tx: Sender<::server::ServerMsg>,
}

impl fmt::Debug for Server {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Server")
            .field("id", &self.id)
            .field("programs", &self.programs)
            .finish()
    }
}

/// A program running on the server
#[derive(Default, Clone, Debug)]
pub struct Program {
    pub id: String,
    /// Whether the client is interested in msgs about this program. If its not visible, the answer
    /// is probably no.
    pub is_subscribed: bool
}

//mod tests {
    //use std::sync::mpsc::channel;
    //use ::server::ServerMsg;

    //use super::*;

    //// Window methods

    //#[test]
    //fn add_window_when_empty_adds_the_window() {
        //let mut state: State = Default::default();
        //let window = Window { id: "saguaro".to_string(), .. Default::default() };
        //state.add_window(window);
        //assert!(state.windows.iter().any(|w| w.id == "saguaro"));
    //}

    //#[test]
    //fn add_window_when_already_added_does_nothing() {
        //let mut state: State = Default::default();
        //let window = Window { id: "saguaro".to_string(), .. Default::default() };
        //state.add_window(window);
        //let window = Window { id: "saguaro".to_string(), .. Default::default() };
        //state.add_window(window);
        //assert_eq!(state.windows.iter().filter(|w| w.id == "saguaro").count(), 1);
    //}

    //#[test]
    //fn remove_window_when_empty_does_nothing() {
        //let mut state: State = Default::default();
        //state.remove_window("unknown");
        //// don't crash
    //}

    //#[test]
    //fn remove_window_when_window_exists_removes_it() {
        //let mut state: State = Default::default();
        //let window = Window { id: "saguaro".to_string(), .. Default::default() };
        //state.add_window(window);

        //state.remove_window("saguaro");
        //assert!(!state.windows.iter().any(|w| w.id == "saguaro"));
    //}

    //// Pane methods

    //#[test]
    //fn add_pane_when_window_doesnt_exist_does_nothing() {
        //let mut state: State = Default::default();
        //let pane = Pane { id: "saguaro".to_string(), .. Default::default() };
        //state.add_pane("uknown", pane);
        //// don't crash
    //}

    //#[test]
    //fn add_pane_when_window_is_empty_adds_the_pane() {
        //let mut state: State = Default::default();
        //let window = Window { id: "red".to_string(), .. Default::default() };
        //state.add_window(window);
        //let pane = Pane { id: "blue".to_string(), .. Default::default() };
        //state.add_pane("red", pane);
        //assert!(state.windows.iter().find(|w| w.id == "red").unwrap().panes.iter().any(|w| w.id == "blue"));
    //}

    //#[test]
    //fn add_pane_when_already_exists_does_nothing() {
        //let mut state: State = Default::default();
        //let window = Window { id: "red".to_string(), .. Default::default() };
        //state.add_window(window);
        //let pane = Pane { id: "blue".to_string(), .. Default::default() };
        //state.add_pane("red", pane);
        //let pane = Pane { id: "blue".to_string(), .. Default::default() };
        //state.add_pane("red", pane);
        //assert_eq!(state.windows.iter().find(|w| w.id == "red").unwrap().panes.iter().filter(|w| w.id == "blue").count(), 1);
    //}

    ////#[test]
    ////fn remove_pane_when_empty_does_nothing() {
        ////let mut state: State = Default::default();
        ////state.remove_pane("unknown");
        ////// don't crash
    ////}

    ////#[test]
    ////fn remove_pane_when_pane_exists_removes_it() {
        ////let mut state: State = Default::default();
        ////let pane = Window { id: "saguaro".to_string(), .. Default::default() };
        ////state.add_pane(window);

        ////state.remove_pane("saguaro");
        ////assert!(!state.windows.iter().any(|w| w.id == "saguaro"));
    ////}

    //// Server methods

    //fn new_server(id: &str) -> Server {
        //let (dummy_tx, dummy_rx) = channel::<ServerMsg>();

        //Server {
            //id: id.to_string(),
            //programs: vec![],
            //tx: dummy_tx,
        //}
    //}

    //#[test]
    //fn add_server_when_empty_adds_the_server() {
        //let mut state: State = Default::default();
        //let server = new_server("saguaro");
        //state.add_server(server);
        //assert!(state.servers.iter().any(|w| w.id == "saguaro"));
    //}

    //#[test]
    //fn add_server_when_already_added_does_nothing() {
        //let mut state: State = Default::default();
        //let server = new_server("saguaro");
        //state.add_server(server);
        //let server = new_server("saguaro");
        //state.add_server(server);
        //assert_eq!(state.servers.iter().filter(|w| w.id == "saguaro").count(), 1);
    //}

    //#[test]
    //fn remove_server_when_empty_does_nothing() {
        //let mut state: State = Default::default();
        //state.remove_server("unknown");
        //// don't crash
    //}

    //#[test]
    //fn remove_server_when_server_exists_removes_it() {
        //let mut state: State = Default::default();
        //let server = new_server("saguaro");
        //state.add_server(server);

        //state.remove_server("saguaro");
        //assert!(!state.servers.iter().any(|w| w.id == "saguaro"));
    //}

    //// Program methods

    //#[test]
    //fn add_program_when_server_doesnt_exist_does_nothing() {
        //let mut state: State = Default::default();
        //let program = Program { id: "saguaro".to_string(), .. Default::default() };
        //state.add_program("uknown", program);
        //// don't crash
    //}

    //#[test]
    //fn add_program_when_server_is_empty_adds_the_program() {
        //let mut state: State = Default::default();
        //let server = new_server("red");
        //state.add_server(server);
        //let program = Program { id: "blue".to_string(), .. Default::default() };
        //state.add_program("red", program);
        //assert!(state.servers.iter().find(|w| w.id == "red").unwrap().programs.iter().any(|w| w.id == "blue"));
    //}

    //#[test]
    //fn add_program_when_already_exists_does_nothing() {
        //let mut state: State = Default::default();
        //let server = new_server("red");
        //state.add_server(server);
        //let program = Program { id: "blue".to_string(), .. Default::default() };
        //state.add_program("red", program);
        //let program = Program { id: "blue".to_string(), .. Default::default() };
        //state.add_program("red", program);
        //assert_eq!(state.servers.iter().find(|w| w.id == "red").unwrap().programs.iter().filter(|w| w.id == "blue").count(), 1);
    //}

    ////#[test]
    ////fn remove_program_when_server_doesnt_exist_does_nothing() {
    ////}

    ////#[test]
    ////fn remove_program_when_server_empty_does_nothing() {
    ////}

    ////#[test]
    ////fn remove_program_when_program_exists_removes_it() {
    ////}

//}
