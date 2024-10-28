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
                let mut servicename: String = String::from("");
                let mut string: String = String::from("");
                let service: &mut Option<Command> = &mut None;
                let mut i = 0;

                for c in line.unwrap().chars() {
                    if c == ':' {
                        servicename = string;
                        string = "".to_string();
                    } else if c ==' ' {
                        if i == 0 {

                        } else if i == 1 {
                            if string != "exec" {
                                string = "".to_string();
                                break;
                            }
                        } else if i == 2 {
                            *service = Some(Command::new(string));
                            string = "".to_string();
                        } else {
                            service.as_mut().unwrap().arg(string);
                            string = "".to_string();
                        }
                        i += 1;
                    } else {
                        string.push(c);
                    }
                }
                if i >= 1 {
                    //if let Ok(child) = service.as_mut().spawn() {
                    //    println!("Lazy: spawn {} {}", servicename, child.id());
                    //    services.insert(servicename, child.id());
                    //} else {
                    //    println!("Lazy: {} failed", servicename);
                    //}
                    if service.is_some() {
                        spawn_service(servicename, &mut service.as_mut().unwrap(), owner);
                    }
                }
            }
        }
    }
}

pub fn main() {
    use std::process::Command;
    use super::server;

    let mut the_owner = TheOwner {services: HashMap::new(), count: HashMap::new()};
    println!("Lazy init");

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

    server::main();
}

