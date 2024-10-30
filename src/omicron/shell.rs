use crate::omicron::command;

pub fn run(_command: &str) -> Result<command::Process, String> {
    use crate::omicron::command::CommandBuilder;
    let mut builder = CommandBuilder::new();

    builder.spawn()
}
