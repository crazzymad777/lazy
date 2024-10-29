pub struct CommandBuilder {
}

impl CommandBuilder {
    pub fn new(program: &str) -> CommandBuilder {
        CommandBuilder {}
    }

    pub fn spawn(&mut self) {
        // Something like that.....
        use crate::omicron::utils::Cstr;
        let s = Cstr::check("sh\0");
        let x = s.unwrap();
        unsafe {
            libc::execv(Cstr::magic(x), std::ptr::null());
        }
    }
}
