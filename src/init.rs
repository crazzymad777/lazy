use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn main() {
    use std::process::Command;
    use super::server;

    println!("Lazy init");

    let path = Path::new("/etc/lazy.d/init");
    if path.exists() {
        if let Ok(lines) = read_lines(path) {
            for line in lines {
                if let Ok(ref x) = line {
                    // name of service: exec cmd (args)
                    let mut servicename: String = String::from("");
                    let mut string: String = String::from("");
                    let mut service: Box<Command> = Box::new(Command::new(""));
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
                                service = Box::new(Command::new(string));
                                string = "".to_string();
                            } else {
                                service.as_mut().arg(string);
                                string = "".to_string();
                            }
                            i += 1;
                        } else {
                            string.push(c);
                        }
                    }
                    if i >= 1 {
                        if let Ok(child) = service.as_mut().spawn() {
                            println!("Lazy: spawn {} {}", servicename, child.id());
                        } else {
                            println!("Lazy: {} failed", servicename);
                        }
                    }
                }
            }
        }
    } else {
        Command::new("agetty").arg("tty1").spawn();
        Command::new("agetty").arg("tty2").spawn();
        Command::new("agetty").arg("tty3").spawn();
        Command::new("agetty").arg("tty4").spawn();
        Command::new("agetty").arg("tty5").spawn();
        Command::new("agetty").arg("tty6").spawn();
        Command::new("/usr/lib/systemd/systemd-udevd").arg("--daemon").spawn();
    }

    server::main();
}

