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

#[cfg(test)]
mod testsCstr {
    use crate::omicron::utils::Cstr;
    use super::*;

    #[test]
    #[should_panic]
    fn test_check_panic() {
        Cstr::check("non-null-terminated-string").ok().unwrap();
    }

    #[test]
    fn test_check() {
        let x = "null-terminated-string\0";
        let expected = x;
        let actual = Cstr::check(x).ok().unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    #[should_panic]
    fn test_new_magic_panic() {
        Cstr::new_magic("non-null-terminated-string");
    }

    #[test]
    fn test_magic() {
        let x = "null-terminated-string\0";
        let expected = x.as_ptr() as *const libc::c_char;
        let actual = Cstr::magic(x);
        assert_eq!(expected, actual);
    }
}

