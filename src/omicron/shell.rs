use crate::omicron::command;

pub fn run(_command: &str) -> Result<command::Process, String> {
    use crate::omicron::command::CommandBuilder;
    let memory = String::with_capacity(256);

    let mut builder = CommandBuilder::new();
    let mut i = 0;
    let len = _command.len();
    let mut toggle = false;
    while i < len {
        let x = _command[i];
        if x == ' ' {
            memory.push('\0');
            if (toggle) {
                builder.arg(memory);
            } else {
                builder.program(memory);
                toggle = true;
            }
            memory = String::from("");
        } else {
            s.push(x);
        }
        i = i + 1;
    }

    if (memory != "") {
        builder.arg(memory);
    }

    builder.spawn()
}
