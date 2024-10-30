use crate::omicron::Process;

#[cfg(test)]
impl ToString for CommandBuilder {
    fn to_string(&self) -> String {
        let mut result = String::with_capacity(256);
        result = result + &self.program;
        result.push(';');
        for x in self.args.iter() {
            result = result + &x;
            result.push(',');
        }
        result.push(';');
        result = result + &self.new_group.to_string();
        return result;
    }
}

// Any &str passed to CommandBuilder must be checked
impl CommandBuilder {
    pub fn new() -> CommandBuilder {
        CommandBuilder {
            program: String::from(""),
            args: Vec::new(),
            new_group: false
        }
    }

    pub fn program(&mut self, program: &str) -> &mut Self {
        use crate::omicron::utils::Cstr;
        Cstr::check(program).unwrap();
        self.program = String::from(program);
        self
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

    pub fn spawn(&self) -> Result<Process, String> {
        use crate::omicron::utils::Cstr;

        // We must provide correct arguments for execute function
        //  - argv[0] = program name
        //  - last element of argv is null pointer

        let l = self.args.len();
        let mut ptr_args: Vec<*const i8> = Vec::with_capacity(l+2);
        let file = Cstr::magic(self.program.as_str());
        ptr_args.push(file); // provide filename of programs as first argument

        let mut i = 0;
        while i < l {
            let x = Cstr::magic(self.args[i].as_str());
            ptr_args.push(x);
            i = i + 1;
        }
        ptr_args.push(std::ptr::null()); // last pointer should be zero

        unsafe {
            // fork, create new session if necessary & execute
            crate::omicron::command::utils::execute(self.program.as_str(), &ptr_args, self.new_group)
        }
    }
}

// &str can be stored in struct if and only if when it was checked

#[derive(Clone)]
pub struct CommandBuilder {
    program: String,
    args: Vec<String>,
    new_group: bool
}

