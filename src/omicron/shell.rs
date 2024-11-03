use crate::omicron::Process;

pub fn run(_command: &str) -> Result<Process, String> {
    use crate::omicron::ShellCommand;
    crate::omicron::command::parse(_command).spawn()
}
