pub fn main() {
    use std::process::Command;
    use super::server;

    println!("Lazy init");

    Command::new("agetty").arg("tty1").spawn();
    Command::new("agetty").arg("tty2").spawn();
    Command::new("agetty").arg("tty3").spawn();
    Command::new("agetty").arg("tty4").spawn();
    Command::new("agetty").arg("tty5").spawn();
    Command::new("agetty").arg("tty6").spawn();
    Command::new("/usr/lib/systemd/systemd-udevd").arg("--daemon").spawn();

    server::main();
}

