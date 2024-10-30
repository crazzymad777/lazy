// Any &str passed to CommandBuilder must be checked
impl CommandBuilder<'_> {
    pub fn new(program: &str) -> CommandBuilder {
        use crate::omicron::utils::Cstr;
        Cstr::check(program).unwrap();
        CommandBuilder {
            program,
            args: Vec::new(),
            new_group: false
        }
    }

    pub fn group(&mut self) -> &mut Self {
        self.new_group = true;
        self
    }

    pub fn arg(&mut self, argument: &str) -> &mut Self {
        crate::omicron::utils::Cstr::check(argument).unwrap();
        self.args.push(String::from(argument));
        self
    }

    pub fn set_args(&mut self, arguments: Vec<&str>) -> &mut Self {
        let l = arguments.len();
        self.args = Vec::with_capacity(l);
        let mut i = 0;
        while i < l {
            let argument = arguments[i]; //.as_str();
            crate::omicron::utils::Cstr::check(argument).unwrap();
            self.args.push(String::from(argument));
            i = i + 1;
        }
        self
    }

    pub fn spawn(&mut self) -> Result<Process, String> {
        use crate::omicron::utils::errno_to_string;
        use crate::omicron::utils::Cstr;

        unsafe {
            let result = libc::fork();

            if result != 0 {
                return if result > 0 {
                    Ok(Process::new(result))
                } else {
                    // result < 0
                    Err(errno_to_string().unwrap_or("fork failed".to_string()))
                }
            }

            if self.new_group {
                libc::setsid();
            }

            // result = 0
            let l = self.args.len();
            let mut args: Vec<*const i8> = Vec::with_capacity(l+2);
            let file = Cstr::magic(self.program);
            args.push(file); // provide filename of programs as first argument

            let mut i = 0;
            while i < l {
                let x = Cstr::magic(self.args[i].as_str());
                args.push(x);
                i = i + 1;
            }
            args.push(std::ptr::null()); // last pointer should be zero

            let _error = libc::execvp(file, args.as_ptr());
            panic!("execvp failed: {}", errno_to_string().unwrap_or("execv failed".to_string())) // child panic
        }
    }
}

// &str can be stored in struct if and only if when it was checked
pub struct CommandBuilder<'a> {
    program: &'a str,
    args: Vec<String>,
    new_group: bool
}

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

