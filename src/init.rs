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

fn spawn_service(servicename: String, command: &mut Command, services: &mut HashMap<String, u32>) {
    if let Ok(child) = command.spawn() {
        println!("Lazy: spawn {} {}", servicename, child.id());
        services.insert(servicename, child.id());
    } else {
        println!("Lazy: {} failed", servicename);
    }
}

fn parse_init_file<P>(path: P, services: &mut HashMap<String, u32>) where P: AsRef<Path> {
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(ref x) = line {
                // name of service: exec cmd (args)
                let mut servicename: String = String::from("");
                let mut string: String = String::from("");
                let mut service: &mut Option<Command> = &mut None;
                let mut i = 0;

                for c in line.unwrap().chars() {
                    if c == ':' {
                        servicename = string;
                        string = "".to_string();
                    } else if c ==' ' {
                        if i == 1 {
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
                        spawn_service(servicename, &mut service.as_mut().unwrap(), services);
                    }
                }
            }
        }
    }
}

pub fn main() {
    use std::process::Command;
    use super::server;

    let mut services: HashMap<String, u32> = HashMap::new();
    println!("Lazy init");

    let path = Path::new("/etc/lazy.d/init");
    if path.exists() {
        parse_init_file(path, &mut services);
    } else {
        spawn_service("agetty1".to_string(), &mut Command::new("agetty").arg("tty1"), &mut services);
        spawn_service("agetty2".to_string(), &mut Command::new("agetty").arg("tty2"), &mut services);
        spawn_service("agetty3".to_string(), &mut Command::new("agetty").arg("tty3"), &mut services);
        spawn_service("agetty4".to_string(), &mut Command::new("agetty").arg("tty4"), &mut services);
        spawn_service("agetty5".to_string(), &mut Command::new("agetty").arg("tty5"), &mut services);
        spawn_service("agetty6".to_string(), &mut Command::new("agetty").arg("tty6"), &mut services);
        spawn_service("udevd".to_string(), &mut Command::new("/usr/lib/systemd/systemd-udevd").arg("--daemon"), &mut services);
    }

    server::main();
}

