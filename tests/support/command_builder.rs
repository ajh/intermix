use std::process::Command;

/// Builds a Command and implements Send, which Command doesn't. Doesn't support all the options of
/// std::process::Command, just the ones that are needed.
pub struct CommandBuilder {
    program: String,
    args: Vec<String>,
}

impl CommandBuilder {
    pub fn new(program: &str) -> CommandBuilder {
        CommandBuilder {
            program: program.to_owned(),
            args: vec![],
        }
    }

    pub fn arg(mut self, arg: &str) -> CommandBuilder {
        self.args.push(arg.to_owned());
        self
    }

    pub fn build(self) -> Command {
        let mut command = Command::new(self.program);
        command.args(self.args.as_slice());
        command
    }
}
