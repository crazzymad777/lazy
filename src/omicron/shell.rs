use crate::omicron::Process;

pub fn run(_command: &str) -> Result<Process, String> {
    crate::omicron::command::parse(_command).spawn()
}
