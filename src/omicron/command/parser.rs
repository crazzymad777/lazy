use crate::omicron::command::CommandBuilder;

pub struct CommandParser {
    memory: String, // use raw bytes?
    builder: CommandBuilder,
    toggle: bool,
    in_single_quotes: bool,
    escape: bool
}

impl CommandParser {
    pub fn new() -> CommandParser {
        CommandParser {
            memory: String::with_capacity(256),
            builder: CommandBuilder::new(),
            toggle: false,
            in_single_quotes: false,
            escape: false
        }
    }

    fn load(&mut self) {
        self.memory.push('\0');
        if self.toggle {
            self.builder.arg(&self.memory);
        } else {
            self.builder.program(&self.memory);
            self.toggle = true;
        }
    }

    pub fn feed_char(&mut self, x: char) {
        if self.in_single_quotes {
            self.feed_char_with_sepataror(x, '\'');
        } else {
            self.feed_char_with_sepataror(x, ' ');
        }
    }

    pub fn feed_char_with_sepataror(&mut self, x: char, separator: char) {
        if x == separator {
            if x == ' ' {
                self.load();
                self.memory = String::from("");
            }
            if x == '\'' {
                self.in_single_quotes = false;
            }
        } else {
            if x == '\'' {
                if self.escape {
                    self.escape = false;
                    self.memory.push(x);
                } else {
                    self.in_single_quotes = true
                }
            } else {
                if x == '\\' && !self.in_single_quotes {
                    if self.escape {
                        self.escape = false;
                        self.memory.push(x);
                    } else {
                        self.escape = true;
                    }
                } else {
                    self.memory.push(x);
                }
            }
        }
    }

    pub fn finish(mut self) -> CommandBuilder {
        if self.memory != "" {
            self.load();
        }
        self.builder
    }
}

pub fn parse(_command: &str) -> CommandBuilder {
    let mut parser = CommandParser::new();

    for x in _command.chars() {
        parser.feed_char(x);
    }
    parser.finish()
}
