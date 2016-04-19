#[derive(Debug)]
pub struct Program {
    id: String,
    command: String,
    pid: i32,
}

impl Program {
    pub fn new<S>(id: S, command: S, pid: i32) -> Program
        where S: Into<String>
    {
        Program {
            id: id.into(),
            command: command.into(),
            pid: pid,
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn set_id<S>(&mut self, value: S) where S: Into<String> {
        self.id = value.into();
    }

    pub fn command(&self) -> &String {
        &self.command
    }

    pub fn set_command<S>(&mut self, value: S) where S: Into<String> {
        self.command = value.into();
    }

    pub fn pid(&self) -> i32 {
        self.pid
    }

    pub fn set_pid(&mut self, value: i32) {
        self.pid = value;
    }
}
