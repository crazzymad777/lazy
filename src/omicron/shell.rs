use crate::omicron::command::CommandBuilder;
use crate::omicron::command;

pub struct CommandParser {
    memory: String, // use raw bytes?
    builder: CommandBuilder,
    toggle: bool
}

impl CommandParser {
    pub fn new() -> CommandParser {
        CommandParser {
            memory: String::with_capacity(256),
            builder: CommandBuilder::new(),
            toggle: false
        }
    }

    fn load(&mut self) {
        self.memory.push('\0');
        if self.toggle {
            self.builder.arg(&self.memory);
        } else {
            self.builder.program(&self.memory);
            self.toggle = true;
        }
    }

    pub fn feed_char(&mut self, x: char) {
        if x == ' ' {
            self.load();
            self.memory = String::from("");
        } else {
            self.memory.push(x);
        }
    }

    pub fn finish(mut self) -> CommandBuilder {
        if self.memory == "" {
            self.load();
        }
        self.builder
    }
}

pub fn parse(_command: &str) -> CommandBuilder {
    let mut parser = CommandParser::new();

    for x in _command.chars() {
        parser.feed_char(x);
    }
    parser.finish()
}

pub fn run(_command: &str) -> Result<command::Process, String> {
    parse(_command).spawn()
}
