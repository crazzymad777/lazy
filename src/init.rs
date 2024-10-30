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

fn parse_init_file<P>(path: P, warden: &mut Warden) where P: AsRef<Path> {

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
                                let mut parser = crate::omicron::command::CommandParser::new();
                                for z in memory.chars() {
                                    parser.feed_char(z);
                                }
                                parser.feed_char(' ');

                                let mut k = 0;
                                for y in string.chars() {
                                    k = k + 1;
                                    if k > j {
                                        parser.feed_char(y);
                                    }
                                }
                                warden.spawn_supervised(&UnitDescriptor::new(servicename.as_str(), parser.finish()));
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
    use super::sys::{provide_hostname, mount_fstab, disable_nologin};
    use std::collections::HashMap;
    use super::server;

    let mut warden = Warden::new(HashMap::new(), HashMap::new());
    println!("Lazy init");
    //init_mount();
    provide_hostname();
    mount_fstab();
    disable_nologin();

    let path = Path::new("/etc/lazy.d/init");
    if path.exists() {
        parse_init_file(path, &mut warden);
    } else {
        // spawn_service("agetty".to_string(), &mut CommandBuilder::new().program("agetty\0").arg("tty1\0"), &mut warden);
        // spawn_service("agetty".to_string(), &mut CommandBuilder::new().program("agetty\0").arg("tty2\0"), &mut warden);
        // spawn_service("agetty".to_string(), &mut CommandBuilder::new().program("agetty\0").arg("tty3\0"), &mut warden);
        // spawn_service("agetty".to_string(), &mut CommandBuilder::new().program("agetty\0").arg("tty4\0"), &mut warden);
        // spawn_service("agetty".to_string(), &mut CommandBuilder::new().program("agetty\0").arg("tty5\0"), &mut warden);
        // spawn_service("agetty".to_string(), &mut CommandBuilder::new().program("agetty\0").arg("tty6\0"), &mut warden);
        //spawn_service("udevd".to_string(), &mut Command::new("/usr/lib/systemd/systemd-udevd").arg("--daemon").group(), &mut the_owner);
    }

    let _ = server::main();
}

