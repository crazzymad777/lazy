use crate::omicron::command::CommandBuilder;
use crate::omicron::command;

pub fn parse(_command: &str) -> CommandBuilder {
    let mut memory = String::with_capacity(256); // use raw bytes?
    let mut builder = CommandBuilder::new();
    let mut i = 0;
    let len = _command.len();
    let mut toggle = false;

    for x in _command.chars() {
        if x == ' ' {
            memory.push('\0');
            if toggle {
                builder.arg(&memory);
            } else {
                builder.program(&memory);
                toggle = true;
            }
            memory = String::from("");
        } else {
            memory.push(x);
        }
    }

    if memory != "" {
        memory.push('\0');
        builder.arg(&memory);
    }

    builder
}

pub fn run(_command: &str) -> Result<command::Process, String> {
    parse(_command).spawn()
}
