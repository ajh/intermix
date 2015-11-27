mod program_mode;

pub use self::program_mode::*;

pub trait Mode {
    fn input(&mut self, bytes: Vec<u8>, client_state: &mut ::client::state::State);

    /// Maybe use the Display trait instead?
    fn display(&self) -> String;
}
