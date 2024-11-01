use std::thread;
use std::os::unix::net::{UnixStream, UnixListener};

fn handle_client(mut stream: UnixStream, tx: std::sync::mpsc::Sender<super::message::Message>) -> std::io::Result<()> {
    
    use std::io::Read;
	use crate::message::Message;
	use crate::message::MessageCommand;
	use crate::message::MessagePayload;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    if response == "poweroff" || response == "halt" || response == "reboot" {
        //sys::reboot(response);
		let message = Message::new(
			MessageCommand::Shutdown,
			MessagePayload::Shutdown(response)
		);
		tx.send(message);
    }
    //if response == "list" || response == "status" || response == "start" || response == "stop" {
	//
	//	}
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

