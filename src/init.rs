use super::unit::descriptor::UnitDescriptor;
use super::warden::Warden;
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

use crate::omicron::command::CommandParser;
use std::sync::mpsc::Sender;
use super::message::Message;

struct InitParser {
    service_name: String,
    memory: String,
    possible_service_name: bool,
    j: u32,
    command_parser: CommandParser
}

impl InitParser {
    pub fn new() -> InitParser {
        InitParser {
            service_name: String::from(""),
            memory: String::from(""),
            possible_service_name: true,
            j: 0,
            command_parser: CommandParser::new()
        }
    }

    pub fn exec_service(&mut self) -> bool {
        self.j >= 1
    }

    pub fn feed_char(&mut self, x: char) {
        if self.possible_service_name {
            if x == ':' {
                self.service_name = self.memory.clone();
                self.possible_service_name = false;
                self.memory.clear();
            } else if x == ' ' {
                self.possible_service_name = false;
            } else {
                self.memory.push(x);
            }
        } else {
            if x == ' ' {
                if self.memory != "" {
                    if self.j == 0 {
                        if self.memory == "exec" {
                            self.command_parser.set_group();
                            self.j += 1;
                        }
                    } else if self.j >= 1 {
                        self.command_parser.feed_char(x);
                        self.j += 1;
                    }
                }
            } else {
                if self.j == 0 {
                    self.memory.push(x);
                } else if self.j >= 1 {
                    self.command_parser.feed_char(x);
                    self.j += 1;
                }
            }
        }
    }
}

fn parse_init_file<P>(path: P, tx: Sender<Message>) where P: AsRef<Path> {
    use crate::message::*;

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(ref result) = line {
                // name of service: exec cmd (args)
                // +mount
                // +hostname
                //

                let mut parser = InitParser::new();
                for x in result.chars() {
                    parser.feed_char(x);
                }

                if parser.exec_service() {
                    let message = Message::new(
                        MessageCommand::ExecService,
                        MessagePayload::Descriptor(UnitDescriptor::new(
                            parser.service_name,
                            parser.command_parser.finish()
                        ))
                    );
                    tx.send(message);
                }
            }
        }
    }
}

pub fn main() {
    use crate::message::Message;

    use std::sync::mpsc::{Sender, Receiver};
    use std::sync::mpsc;
    use std::thread;

    use super::sys::{provide_hostname, mount_fstab, disable_nologin};
    use std::collections::HashMap;
    use super::server;

    println!("Lazy init");

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        super::warden::spawn_warden(rx);
    });

    //init_mount();
    provide_hostname();
    mount_fstab();
    disable_nologin();

    let path = Path::new("/etc/lazy.d/init");
    if path.exists() {
        parse_init_file(path, tx.clone());
    } else {
        use crate::omicron::command::CommandBuilder;
        use crate::message::*;

        let mut builder = CommandBuilder::new();
        builder.program("agetty\0").arg("tty1\0").group();

        let message = Message::new(
            MessageCommand::ExecService,
            MessagePayload::Descriptor(UnitDescriptor::new(
                "agetty".to_string(),
                builder
            ))
        );
        tx.send(message);
    }

    let _ = server::main(tx);
}

