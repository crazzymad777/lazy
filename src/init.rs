use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

use std::process::{Command, Child};
use std::collections::HashMap;

struct TheOwner {
    services: HashMap<String, Child>,
    count: HashMap<String, u32>
}

impl TheOwner {
    fn generate_name(&mut self, servicename: String) -> String {
        let mut string: String = servicename.clone();
        let option = self.count.get_mut(&string);
        if option.is_some() {
             let value = option.unwrap();
             string = string + &value.to_string();
             *value = *value+1;
        } else {
            self.count.insert(string.clone(), 1);
        }
        string
    }

    fn save(&mut self, servicename: String, child: Child) {
        self.services.insert(servicename, child);
    }
}

fn spawn_service(servicename: String, command: &mut Command, owner: &mut TheOwner) {
    let name = owner.generate_name(servicename);
    if let Ok(child) = command.spawn() {
        println!("Lazy: spawn {} {}", name, child.id());
        owner.save(name, child);
    } else {
        println!("Lazy: {} failed", name);
    }
}

fn parse_init_file<P>(path: P, owner: &mut TheOwner) where P: AsRef<Path> {
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(ref _x) = line {
                // name of service: exec cmd (args)
                // +mount
                // +hostname
                //

                // The Worst Parser Ever
                let mut servicename = String::from("unit");
                let mut memory = String::from("");
                let string = line.unwrap();
                let mut i = 0;
                for x in string.chars() {
                    i += 1;
                    if x == ':' {
                        servicename = memory.clone();
                        break;
                    }
                    if x == ' ' {
                        i = 0;
                        break;
                    }
                    memory.push(x);
                }

                memory = String::from("");
                let mut j = 0;
                //println!("servicename: {}", servicename);
                for x in string.chars() {
                    j += 1;
                    if j > i {
                        if x == ' ' {
                            if memory == "" {

                            } else if memory == "exec" {

                            } else {
                                //println!("command: {}", memory);
                                let mut service = Command::new(memory);
                                memory = String::from("");
                                let mut k = 0;
                                for y in string.chars() {
                                    k += 1;
                                    if k > j {
                                        if y == ' ' {
                                            //println!("load arg: {}", memory);
                                            service.arg(memory);
                                            memory = String::from("");
                                        } else {
                                            memory.push(y);
                                        }
                                    }
                                }
                                if memory != "" {
                                    //println!("load arg: {}", memory);
                                    service.arg(memory);
                                }
                                spawn_service(servicename, &mut service, owner);
                                break;
                            }
                            memory = String::from("");
                            continue;
                        }
                        memory.push(x);
                    }
                }
            }
        }
    }
}

pub fn main() {
    use std::process::Command;
    use super::server;
    use crate::sys::{init_mount, provide_hostname};

    let mut the_owner = TheOwner {services: HashMap::new(), count: HashMap::new()};
    println!("Lazy init");
    init_mount();
    provide_hostname();

    let path = Path::new("/etc/lazy.d/init");
    if path.exists() {
        parse_init_file(path, &mut the_owner);
    } else {
        spawn_service("agetty".to_string(), &mut Command::new("agetty").arg("tty1"), &mut the_owner);
        spawn_service("agetty".to_string(), &mut Command::new("agetty").arg("tty2"), &mut the_owner);
        spawn_service("agetty".to_string(), &mut Command::new("agetty").arg("tty3"), &mut the_owner);
        spawn_service("agetty".to_string(), &mut Command::new("agetty").arg("tty4"), &mut the_owner);
        spawn_service("agetty".to_string(), &mut Command::new("agetty").arg("tty5"), &mut the_owner);
        spawn_service("agetty".to_string(), &mut Command::new("agetty").arg("tty6"), &mut the_owner);
        spawn_service("udevd".to_string(), &mut Command::new("/usr/lib/systemd/systemd-udevd").arg("--daemon"), &mut the_owner);
    }

    let _ = server::main();
}

