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

    pub fn spawn(&mut self) {
        use crate::omicron::utils::Cstr;
        unsafe {
            libc::execv(Cstr::magic(self.program), std::ptr::null());
        }
    }
}

// &str can be stored in struct if and only if when it was checked
pub struct CommandBuilder<'a> {
    program: &'a str,
    args: Vec<&'a str>,
    new_group: bool
}
