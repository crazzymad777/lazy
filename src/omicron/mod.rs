// Omicron Shell Executor
//pub struct CommandBuilder;

pub mod command;
pub mod utils;
pub mod shell;


#[derive(Copy, Clone)]
pub struct Process {
    id: libc::pid_t
}

impl Process {
    fn new(id: libc::pid_t) -> Process {
        //unsafe {let pgid = libc::getpgid(id);}
        Process {id}
    }

    pub fn id(self: Process) -> libc::pid_t {
        self.id
    }

    pub fn signal(self: Process, signal: i32) -> Result<(), String> {
        use crate::omicron::utils::errno_to_string;
        unsafe {
            if libc::kill(self.id, signal) == -1 {
                Err(errno_to_string().unwrap_or("kill failed".to_string()))
            } else {
                Ok(())
            }
        }
    }
}
