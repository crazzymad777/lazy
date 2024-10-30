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
        use crate::omicron::utils::Cstr;
        use libc::c_char;
        unsafe {
            let result = libc::fork();
            if result == 0 {
                libc::execv(Cstr::magic(self.program), std::ptr::null());
                // never reached
            } else if result == -1 {
                let buffer: [u8; 256] = [0; 256];
                let pointer = buffer.as_ptr() as *mut c_char;
                let current_error = *libc::__errno_location();

                if libc::strerror_r(current_error, pointer, 256) == 0 {
                    let e = std::str::from_utf8(buffer.as_slice());
                    return Err(String::from(e.unwrap_or("fork failed")));
                }
                return Err(String::from("fork failed"));
            }
            return Ok(result);
        }
    }
}

// &str can be stored in struct if and only if when it was checked
pub struct CommandBuilder<'a> {
    program: &'a str,
    args: Vec<&'a str>,
    new_group: bool
}
