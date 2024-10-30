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
        //self.args.push(argument);
        self
    }

    pub fn spawn(&mut self) -> Result<libc::pid_t, String> {
        use crate::omicron::utils::errno_to_string;
        use crate::omicron::utils::Cstr;
        use libc::c_char;

        unsafe {
            let result = libc::fork();

            if result != 0 {
                return if result > 0 {
                    Ok(result)
                } else {
                    // result < 0
                    Err(errno_to_string().unwrap_or("fork failed".to_string()))
                }
            }

            // result = 0
            let error = libc::execv(Cstr::magic(self.program), std::ptr::null());
            panic!("execv failed: {}", errno_to_string().unwrap_or("execv failed".to_string())) // child panic
        }
    }
}

// &str can be stored in struct if and only if when it was checked
pub struct CommandBuilder<'a> {
    program: &'a str,
    args: Vec<&'a str>,
    new_group: bool
}
