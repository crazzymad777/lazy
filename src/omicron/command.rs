pub struct CommandBuilder<'a> {
    program: &'a str
}

impl CommandBuilder<'_> {
    pub fn new(program: &str) -> CommandBuilder {
        CommandBuilder {program}
    }

    pub fn spawn(&mut self) {
        // Something like that.....
        use crate::omicron::utils::Cstr;
        unsafe {
            libc::execv(Cstr::new_magic("sh\0"), std::ptr::null());
        }
    }
}
