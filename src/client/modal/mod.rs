mod program_mode;

pub use self::program_mode::*;
use std::fmt::Debug;

pub trait Mode : Debug {
    fn input(&self, bytes: Vec<u8>, windows: &mut ::client::state::Windows, servers: &mut ::client::state::Servers);

    /// Maybe use the Display trait instead?
    fn display(&self) -> String;
}
