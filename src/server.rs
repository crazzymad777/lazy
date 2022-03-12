use std::thread;
use std::os::unix::net::{UnixStream, UnixListener};

fn handle_client(mut stream: UnixStream) -> std::io::Result<()> {
    use super::sys;
    use std::io::Read;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    if response == "poweroff" || response == "halt" || response == "restart" {
        sys::reboot(response);
    }
    Ok(())
}

pub fn main() -> std::io::Result<()> {
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

