use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn main() {
    use std::fs::read_to_string;
    use std::process::Command;
    use super::server;

    println!("Lazy init");

    let path = Path::new("/etc/lazy.d/init");
    if path.exists() {
        // for line in read_to_string("/etc/lazy.d/init").unwrap().lines() {
        //     println!("{}", line.to_string());
        // }

        if let Ok(lines) = read_lines(path) {
            for line in lines {
                if let Ok(x) = line {
                    println!("{}", x);
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

