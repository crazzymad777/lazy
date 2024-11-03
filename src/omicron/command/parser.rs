use crate::omicron::command::ShellCommandBuilder;

pub struct CommandParser {
    memory: String, // use raw bytes?
    builder: ShellCommandBuilder,
    toggle: bool,
    in_single_quotes: bool,
    escape: bool
}

impl CommandParser {
    pub fn new() -> CommandParser {
        CommandParser {
            memory: String::with_capacity(256),
            builder: ShellCommandBuilder::new(),
            toggle: false,
            in_single_quotes: false,
            escape: false
        }
    }

    pub fn set_group(&mut self) {
        self.builder.group();
    }

    pub fn set_no_group(&mut self) {
        self.builder.no_group();
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

    fn feed_char_no_sepataror(&mut self, x: char) {
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
                if x == '|' {
                    self.builder.pipe();
                } else {
                    self.memory.push(x);
                }
            }
        }
    }

    pub fn feed_char_with_sepataror(&mut self, x: char, separator: char) {
        if (x == ' ' && x.is_whitespace()) || x == separator {
            if x.is_whitespace() {
                self.load();
                self.memory = String::from("");
            }
            if x == '\'' {
                self.in_single_quotes = false;
            }
        } else {
            self.feed_char_no_sepataror(x);
        }
    }

    pub fn finish(mut self) -> ShellCommandBuilder {
        if self.memory != "" {
            self.load();
        }
        self.builder
    }
}

pub fn parse(_command: &str) -> ShellCommandBuilder {
    let mut parser = CommandParser::new();

    for x in _command.chars() {
        parser.feed_char(x);
    }
    parser.finish()
}
