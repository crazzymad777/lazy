use std::thread;
use std::os::unix::net::{UnixStream, UnixListener};

fn shutdown(response: String) {
    unsafe {
        use syscalls::syscall;
	let cmd = match response.as_str() {
            "poweroff" => 0x4321fedcusize,
            "restart" => 0x01234567usize,
            "halt" => 0xcdef0123usize,
	    &_ => todo!()
	};
        if let Err(e) = syscall(syscalls::SYS_reboot, &syscalls::SyscallArgs::from(&[0xfee1dead, 537993216, cmd])) {
            println!("{}", e);
        }
    }
}

fn handle_client(mut stream: UnixStream) -> std::io::Result<()> {
    use std::io::Read;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    if response == "poweroff" || response == "halt" || response == "restart" {
        shutdown(response);
    }
    Ok(())
}

fn init() -> std::io::Result<()> {
    use std::process::Command;
    use std::os::unix::process::CommandExt;

    println!("Lazy init");

    Command::new("agetty").arg("tty1").spawn();
    //Command::new("agetty tty2").spawn();
    //Command::new("agetty tty3").spawn();
    //Command::new("agetty tty4").spawn();
    //Command::new("agetty tty5").spawn();

    let listener = UnixListener::bind("/run/lazy")?;

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                /* connection succeeded */
                thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                /* connection failed */
                break;
            }
        }
    }
    Ok(())
}

fn lazycmd() -> std::io::Result<()> {
   use std::env;
   use std::process;
   use std::io::Write;

   println!("My pid is {}", process::id());

   let args: Vec<String> = env::args().collect();
   let arg = &args[1];
   let mut stream = UnixStream::connect("/run/lazy")?;
   stream.write_all(arg.as_bytes())?;
   Ok(())
}

fn main() {
    use std::process;

    if process::id() == 1 {
        init();
    } else {
        lazycmd();
    }
}
