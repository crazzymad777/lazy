use std::thread;
use std::os::unix::net::{UnixStream, UnixListener};

fn handle_client(mut stream: UnixStream, tx: std::sync::mpsc::Sender<super::message::Message>) -> std::io::Result<()> {
    use super::sys;
    use std::io::Read;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    if response == "poweroff" || response == "halt" || response == "reboot" {
        sys::reboot(response);
    }
    Ok(())
}

pub fn main(tx: std::sync::mpsc::Sender<super::message::Message>) -> std::io::Result<()> {
	let listener = UnixListener::bind("/run/lazy")?;

	// accept connections and process them, spawning a new thread for each one
	for stream in listener.incoming() {
		match stream {
		    Ok(stream) => {
				/* connection succeeded */
				let messager = tx.clone();
				thread::spawn(|| handle_client(stream, messager));
		    }
		    Err(err) => {
				/* connection failed */
				eprintln!("Lazie Error: {}", err);
				break;
		    }
		}
	}
	Ok(())
}

